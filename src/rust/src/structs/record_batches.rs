//! This module includes the structs, traits, and functions needed to receive a
//! serialized RecordBatch (in IPC format) from R, transform it to a usable
//! RecordBatch, serialize back to IPC format, and send the serialized
//! RecordBatch back to R.

use arrow::{
    array::ArrayDataRef,
    buffer::Buffer,
    datatypes::{DataType, Schema, SchemaRef},
    ipc::reader::StreamReader,
    ipc::writer::{DictionaryTracker, EncodedData},
    record_batch::RecordBatch,
};
use extendr_api::prelude::{FromRobj, Raw, Robj};
use flatbuffers::FlatBufferBuilder;
use std::io::{BufWriter, Write};

pub const PREFIX_SIZE: usize = 8; // Bytes in buffer prefix
pub const BLOCK_SIZE: usize = 8;

type Result<T> = std::result::Result<T, &'static str>;

/// Newtype wrapping a Vec<RecordBatch>
#[derive(Debug)]
pub struct RecordBatches(Vec<RecordBatch>);

impl RecordBatches {
    /// Fetches the schema from the first RecordBatch in RecordBatches
    pub fn schema(&self) -> Option<SchemaRef> {
        self.0.get(0).map(|x| x.schema())
    }

    /// Converts this RecordBatches into an Robj representing a the wrapped
    /// Vec<RecordBatch> as a serialized raw vector. The contents of this function,
    /// as well as most everything under the 'Utility Functions' section, will
    /// need to be updated with Arrow 4.0.0 and the addition of the
    /// `StreamWriter.into_inner()` function.
    pub fn into_robj(self) -> Result<Robj> {
        // TODO: Update when Arrow 4.0.0 is released to crates.io
        let out: Vec<u8> = Vec::new();
        let schema = match self.schema() {
            Some(x) => x,
            None => return Ok(Robj::from(out)),
        };
        let mut dictionary_tracker = DictionaryTracker::new(false);
        let mut writer = BufWriter::new(out);
        let schema_bytes = schema_to_bytes(&schema);
        write_encoded_data_to_stream(&mut writer, schema_bytes)?;

        for batch in self {
            append_record_batch_to_stream(&mut writer, &mut dictionary_tracker, &batch)?;
        }

        // Mark the stream as 'done'
        write_continuation_marker(&mut writer, 0)?;
        let buffer = writer
            .into_inner()
            .map_err(|_| "Could not retrieve writer from BufWriter")?;
        let raw = Raw(&buffer);
        Ok(Robj::from(raw))
    }
}

impl<'a> FromRobj<'a> for RecordBatches {
    /// For converting `Robj` to `RecordBatches`. Expects a raw vector input and
    /// uses a [StreamReader](arrow::ipc::reader::StreamReader) to read that raw
    /// (byte) vector into a set of RecordBatches.
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(slice) = robj.as_raw_slice() {
            let mut v = Vec::new();
            let reader = StreamReader::try_new(slice).map_err(|_| "Error creating StreamReader")?;
            for rb in reader {
                let rb = rb.map_err(|_| "Error reading RecordBatch")?;
                v.push(rb);
            }
            Ok(RecordBatches(v))
        } else {
            Err("Expected a raw buffer type.")
        }
    }
}

/// For converting `RecordBatches` to `Robj`
impl From<RecordBatches> for Robj {
    fn from(batches: RecordBatches) -> Robj {
        batches
            .into_robj()
            .expect("Error converting RecordBatches to Raw/Robj")
    }
}

/// Allow for iterating over the Vec<RecordBatch> wrapped by RecordBatches
impl IntoIterator for RecordBatches {
    type Item = RecordBatch;
    type IntoIter = std::vec::IntoIter<RecordBatch>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// -=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-
// Utility Functions for RecordBatches -----------------------------------------
// -=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-=+=-

// Most of what's in this section comes from inner methods in arrow::ipc::writer::StreamWrtier
// and arrow::ipc::writer::IpcDataGenerator. ALL of this will be replaced with the
// release of Arrow 4.0.0 and the `StreamWriter.into_inner()` function. Feel free
// to peruse these, but 90% of this code is lightly refactored from the
// [arrow::ipc::writer](https://docs.rs/arrow/3.0.0/arrow/ipc/writer/index.html)
// source.

/// Encode a RecordBatch and append its data to the end of the stream
pub fn append_record_batch_to_stream<W: Write>(
    mut stream_writer: W,
    mut dictionary_tracker: &mut DictionaryTracker,
    batch: &RecordBatch,
) -> Result<()> {
    let (encoded_dictionaries, encoded_message) = encode_batch(&batch, &mut dictionary_tracker)
        .map_err(|_| "Could not encode record batch")?;
    for encoded_dictionary in encoded_dictionaries {
        write_encoded_data_to_stream(&mut stream_writer, encoded_dictionary)
            .map_err(|_| "Cound not write message")?;
    }
    write_encoded_data_to_stream(&mut stream_writer, encoded_message)
        .map_err(|_| "Could not write message")?;
    Ok(())
}

pub fn encode_batch(
    batch: &RecordBatch,
    dictionary_tracker: &mut DictionaryTracker,
) -> Result<(Vec<EncodedData>, EncodedData)> {
    let schema = batch.schema();
    let mut encoded_dictionaries = Vec::with_capacity(schema.fields().len());

    for (i, field) in schema.fields().iter().enumerate() {
        let column = batch.column(i);

        if let DataType::Dictionary(_key_type, _value_type) = column.data_type() {
            let dict_id = field
                .dict_id()
                .expect("All Dictionary types have `dict_id`");
            let dict_data = column.data();
            let dict_values = &dict_data.child_data()[0];

            let emit = dictionary_tracker
                .insert(dict_id, column)
                .map_err(|_| "Could not insert dict column into dictionary_tracker")?;

            if emit {
                encoded_dictionaries.push(dictionary_batch_to_bytes(dict_id, dict_values));
            }
        }
    }

    let encoded_message = record_batch_to_bytes(batch);

    Ok((encoded_dictionaries, encoded_message))
}

fn dictionary_batch_to_bytes(dict_id: i64, array_data: &ArrayDataRef) -> EncodedData {
    let mut fbb = FlatBufferBuilder::new();
    let mut nodes: Vec<arrow::ipc::FieldNode> = vec![];
    let mut buffers: Vec<arrow::ipc::Buffer> = vec![];
    let mut arrow_data: Vec<u8> = vec![];

    write_array_data(
        &array_data,
        &mut buffers,
        &mut arrow_data,
        &mut nodes,
        0,
        array_data.len(),
        array_data.null_count(),
    );

    // write data
    let buffers = fbb.create_vector(&buffers);
    let nodes = fbb.create_vector(&nodes);

    let root = {
        let mut batch_builder = arrow::ipc::RecordBatchBuilder::new(&mut fbb);
        batch_builder.add_length(array_data.len() as i64);
        batch_builder.add_nodes(nodes);
        batch_builder.add_buffers(buffers);
        batch_builder.finish()
    };

    let root = {
        let mut batch_builder = arrow::ipc::DictionaryBatchBuilder::new(&mut fbb);
        batch_builder.add_id(dict_id);
        batch_builder.add_data(root);
        batch_builder.finish().as_union_value()
    };

    let root = {
        let mut message_builder = arrow::ipc::MessageBuilder::new(&mut fbb);
        message_builder.add_version(arrow::ipc::MetadataVersion::V5);
        message_builder.add_header_type(arrow::ipc::MessageHeader::DictionaryBatch);
        message_builder.add_bodyLength(arrow_data.len() as i64);
        message_builder.add_header(root);
        message_builder.finish()
    };

    fbb.finish(root, None);
    let finished_data = fbb.finished_data();

    EncodedData {
        ipc_message: finished_data.to_vec(),
        arrow_data,
    }
}

fn record_batch_to_bytes(batch: &RecordBatch) -> EncodedData {
    let mut fbb = FlatBufferBuilder::new();
    let mut nodes: Vec<arrow::ipc::FieldNode> = vec![];
    let mut buffers: Vec<arrow::ipc::Buffer> = vec![];
    let mut arrow_data: Vec<u8> = vec![];
    let mut offset = 0;
    for array in batch.columns() {
        let array_data = array.data();
        offset = write_array_data(
            &array_data,
            &mut buffers,
            &mut arrow_data,
            &mut nodes,
            offset,
            array.len(),
            array.null_count(),
        );
    }

    // write data
    let buffers = fbb.create_vector(&buffers);
    let nodes = fbb.create_vector(&nodes);

    let root = {
        let mut batch_builder = arrow::ipc::RecordBatchBuilder::new(&mut fbb);
        batch_builder.add_length(batch.num_rows() as i64);
        batch_builder.add_nodes(nodes);
        batch_builder.add_buffers(buffers);
        let b = batch_builder.finish();
        b.as_union_value()
    };
    // create an ipc::Message
    let mut message = arrow::ipc::MessageBuilder::new(&mut fbb);
    message.add_version(arrow::ipc::MetadataVersion::V5);
    message.add_header_type(arrow::ipc::MessageHeader::RecordBatch);
    message.add_bodyLength(arrow_data.len() as i64);
    message.add_header(root);
    let root = message.finish();
    fbb.finish(root, None);
    let finished_data = fbb.finished_data();

    EncodedData {
        ipc_message: finished_data.to_vec(),
        arrow_data,
    }
}

fn schema_to_bytes(schema: &Schema) -> EncodedData {
    let mut fbb = FlatBufferBuilder::new();
    let schema = {
        let fb = arrow::ipc::convert::schema_to_fb_offset(&mut fbb, schema);
        fb.as_union_value()
    };

    let mut message = arrow::ipc::MessageBuilder::new(&mut fbb);
    message.add_version(arrow::ipc::MetadataVersion::V5);
    message.add_header_type(arrow::ipc::MessageHeader::Schema);
    message.add_bodyLength(0);
    message.add_header(schema);
    let data = message.finish();
    fbb.finish(data, None);

    let data = fbb.finished_data();
    EncodedData {
        ipc_message: data.to_vec(),
        arrow_data: vec![],
    }
}

fn write_array_data(
    array_data: &ArrayDataRef,
    mut buffers: &mut Vec<arrow::ipc::Buffer>,
    mut arrow_data: &mut Vec<u8>,
    mut nodes: &mut Vec<arrow::ipc::FieldNode>,
    offset: i64,
    num_rows: usize,
    null_count: usize,
) -> i64 {
    let mut offset = offset;
    nodes.push(arrow::ipc::FieldNode::new(
        num_rows as i64,
        null_count as i64,
    ));
    // NullArray does not have any buffers, thus the null buffer is not generated
    if array_data.data_type() != &DataType::Null {
        // write null buffer if exists
        let null_buffer = match array_data.null_buffer() {
            None => {
                // create a buffer and fill it with valid bits
                let num_bytes = arrow::util::bit_util::ceil(num_rows, 8);
                let buffer = arrow::buffer::MutableBuffer::new(num_bytes);
                let buffer = buffer.with_bitset(num_bytes, true);
                buffer.into()
            }
            Some(buffer) => buffer.clone(),
        };

        offset = write_buffer(&null_buffer, &mut buffers, &mut arrow_data, offset);
    }

    array_data.buffers().iter().for_each(|buffer| {
        offset = write_buffer(buffer, &mut buffers, &mut arrow_data, offset);
    });

    if !matches!(array_data.data_type(), DataType::Dictionary(_, _)) {
        // recursively write out nested structures
        array_data.child_data().iter().for_each(|data_ref| {
            // write the nested data (e.g list data)
            offset = write_array_data(
                data_ref,
                &mut buffers,
                &mut arrow_data,
                &mut nodes,
                offset,
                data_ref.len(),
                data_ref.null_count(),
            );
        });
    }

    offset
}

/// Write a buffer to a vector of bytes, and add its ipc::Buffer to a vector
fn write_buffer(
    buffer: &Buffer,
    buffers: &mut Vec<arrow::ipc::Buffer>,
    arrow_data: &mut Vec<u8>,
    offset: i64,
) -> i64 {
    let len = buffer.len();
    let pad_len = pad_to_8(len);
    let total_len: i64 = (len + pad_len) as i64;
    buffers.push(arrow::ipc::Buffer::new(offset, total_len));
    arrow_data.extend_from_slice(buffer.as_slice());
    arrow_data.extend_from_slice(&vec![0u8; pad_len][..]);
    offset + total_len
}

/// Write the contents of an EncodedData struct to a stream, including the IPC
/// message and RecordBatch data
pub fn write_encoded_data_to_stream<W: Write>(
    mut writer: W,
    encoded: EncodedData,
) -> Result<(usize, usize)> {
    let arrow_data_len = encoded.arrow_data.len();
    if arrow_data_len % 8 != 0 {
        return Err("Arrow data not aligned");
    }

    let a = BLOCK_SIZE - 1;
    let buffer = encoded.ipc_message;
    let flatbuf_size = buffer.len();
    let aligned_size = (flatbuf_size + PREFIX_SIZE + a) & !a;
    let padding_bytes = aligned_size - flatbuf_size - PREFIX_SIZE;

    write_continuation_marker(&mut writer, (aligned_size - PREFIX_SIZE) as i32)?;

    // write the flatbuf
    if flatbuf_size > 0 {
        writer
            .write_all(&buffer)
            .map_err(|_| "Could not write buffer.")?;
    }
    pad_output_stream(&mut writer, padding_bytes)?;

    // write arrow data
    let body_len = if arrow_data_len > 0 {
        write_bytes_to_stream(&mut writer, &encoded.arrow_data)?
    } else {
        0
    };

    Ok((aligned_size, body_len))
}

/// Write a 'continuation' marker and the length of the record batch to a
/// stream. This is written prior to the contents of the record batch.
fn write_continuation_marker<W: Write>(mut writer: W, total_len: i32) -> Result<()> {
    writer
        .write_all(&[0xff; 4])
        .map_err(|_| "Could not write continuation marker to buffer.")?;
    writer
        .write_all(&total_len.to_le_bytes()[..])
        .map_err(|_| "Could not write length to buffer.")?;
    writer.flush().map_err(|_| "Could not flush buffer.")?;
    Ok(())
}

/// Write bytes to a stream, padding up to the nearest multliple of 8
fn write_bytes_to_stream<W: Write>(mut writer: W, data: &[u8]) -> Result<usize> {
    let len = data.len();
    let pad_len = pad_to_8(len);
    let total_len = len + pad_len;

    writer
        .write_all(data)
        .map_err(|_| "Could not write data to buffer.")?;
    if pad_len > 0 {
        pad_output_stream(&mut writer, pad_len)?;
    }

    writer.flush().map_err(|_| "Could not flush buffer.")?;
    Ok(total_len)
}

/// Add 0's to the end of a stream up to the nearest multiple of 8 bytes
fn pad_output_stream<W: Write>(mut writer: W, pad_len: usize) -> Result<()> {
    writer
        .write_all(&vec![0u8; pad_len][..])
        .map_err(|_| "Could not write padding to buffer.")?;
    Ok(())
}

/// Rounds a u32 up to a multiple of 8
#[inline]
fn pad_to_8(len: usize) -> usize {
    (((len + 7) & !7) - len) as usize
}
