use crate::structs::{CharVec, RecordBatches};
use extendr_api::prelude::*;

mod algos;
mod passing_values;
mod structs;
mod utils;

/// Multiplies an integer by 10 in Rust
///
/// Demonstrates passing an integer back and forth between R and
/// Rust, multiplies the given integer by 10 in Rust to prove it works.
///
/// The value passed to this fuction must be an integer (from as.integer(n)) or
/// an NA_integer_. The Rust function signature is
/// `fn pass_single_integer(n: Option<i32>) -> Option<i32>`, with NA's being
/// handled as `None` in Rust. The native R integer is also a signed 32-bit
/// integer, which is equivalent to Rust's `i32`.
///
/// @param n An integer value to treat as an i32 in Rust
/// @return n * 10
///
/// @examples
/// pass_single_integer(as.integer(25))  # 250
/// pass_single_integer(NA_integer_)     # NA_integer_
///
/// @export
#[extendr]
fn pass_single_integer(n: Option<i32>) -> Option<i32> {
    passing_values::pass_single_integer_fn(n)
}

/// Multiplies an integer vector by 10 in Rust
///
/// Demonstrates passing an integer vector back and forth between R and Rust,
/// multiplies each element by 10 in Rust to prove it works.
///
/// The vector passed to this function must be an integer (from as.integer(n)) or
/// an NA_integer_. The Rust function signature is
/// `fn pass_multiple_integers(n: Int) -> Int`. In the current version of
/// extendr-api (0.2.0), NA's are implicitly converted to 0's, so if you need
/// NA handling you'll need to take care of that yourself for now.
///
/// @param n An integer vector to treat as an extendr_api::Int in Rust
/// @return n * 10
///
/// @examples
/// pass_multiple_integers(as.integer(c(5, 7, 9)))  # c(50, 70, 90)
/// pass_multiple_integers(as.integer(30))          # 300
/// pass_multiple_integers(NA_integer_)             # 0
///
/// @export
#[extendr]
fn pass_multiple_integers(n: Int) -> Int {
    passing_values::pass_multiple_integers_fn(n)
}

/// Multiplies a double by 10 in Rust
///
/// Demonstrates passing a double value back and forth between R and Rust,
/// multiplies it by 10 in Rust to prove it works.
///
/// The value passed to this function must be a double, which is the default
/// numeric type in R, or an NA_real_. The Rust function signature is
/// `fn pass_single_float_fn(f: Option<f64>) -> Option<f64>`, with NA's being
/// handled as `None` in Rust. The native R double is double-precision floating
/// point number, equivalent to Rust's `f64`.
///
/// @param f A double (float) value to treat as an f64 in Rust
/// @return f * 10
///
/// @examples
/// pass_single_double(5)         # 50
/// pass_single_double(5.5)       # 55
/// pass_single_double(NA_real_)  # NA_real_
///
/// @export
#[extendr]
fn pass_single_double(f: Option<f64>) -> Option<f64> {
    passing_values::pass_single_double_fn(f)
}

/// Multiplies a double vector by 10 in Rust
///
/// Demonstrates passing a double vector back and forth between R and Rust,
/// multiplies each element by 10 in Rust to prove it works.
///
/// The vector passed to this function must be a double vector (default numeric
/// vector type in R) or an NA_real_. The Rust function signature is
/// `fn pass_multiple_doubles_fn(f: Real) -> Real`. NA's, NaN's, Inf's, and
/// -Inf's are handled implicitly by the f64 specification.
///
/// @param f A double (float) vector to treat as an extendr_api::Real in Rust
/// @return f * 10
///
/// @examples
/// pass_multiple_doubles(c(5.55, NA, NaN, Inf, -Inf))  # c(55.5, NA, NaN, Inf, -Inf)
///
/// @export
#[extendr]
fn pass_multiple_doubles(f: Real) -> Real {
    passing_values::pass_multiple_doubles_fn(f)
}

/// Flips a logical value in Rust
///
/// Demonstrates passing a logical (boolean) value back and forth between R and
/// Rust, flipping the value in Rust to prove it works.
///
/// The value passed to this function must be a logical, the boolean type in
/// R, or an NA (plain NA is logical). The Rust function signature is
/// `fn pass_single_logical_fn(b: Option<bool>) -> Option<bool>`, with
/// NA's being handled as `None` in Rust.
///
/// @param b A logical (boolean) value to treat as a bool in Rust
/// @return !b
///
/// @examples
/// pass_single_logical(TRUE)  # FALSE
/// pass_single_logical(NA)    # NA
///
/// @export
#[extendr]
fn pass_single_logical(b: Option<bool>) -> Option<bool> {
    passing_values::pass_single_logical_fn(b)
}

/// Flip a logical vector in Rust
///
/// Demonstrates passing a logical (boolean) vector back and forth between R and
/// Rust, flipping the values in Rust to prove it works.
///
/// The vector passed to this function must be contain logical values, the
/// boolean type in R, or NAs (plain NA is logical). The Rust function
/// signature is `fn pass_multiple_logicals_fn(b: Logical) -> Logical`, with
/// NA's being handled as `None` in Rust.
///
/// @param b A logical (boolean) vector to treat as an extender_api::Logical in Rust
/// @return !b
///
/// @examples
/// pass_multiple_logicals(c(TRUE, NA, FALSE))  # c(FALSE, NA, TRUE)
///
/// @export
#[extendr]
fn pass_multiple_logicals(b: Logical) -> Logical {
    passing_values::pass_multiple_logicals_fn(b)
}

/// Left shift a raw value in Rust
///
/// Demonstrates passing a raw (byte) value back and forth between R and
/// Rust, left shifting the value in Rust to prove it works.
///
/// The value passed to this function must be a raw, the byte type in R. The
/// Rust function signature is `fn pass_single_raw_fn(r: u8) -> u8`. R does
/// not support NAs for raw types. Currently, extendr (v0.2.0) supports passing
/// in numbers from 0-255 instead of values from `as.raw()` as single values.
/// Values below 0 are coerced to 0, values greater than 255 are coerced to
/// 255.
///
/// @param r A raw value to treat as a byte in Rust
/// @return bitwShiftL(r, 1)
///
/// @examples
/// pass_single_raw(4)    # 8
/// pass_single_raw(-10)  # 0
/// pass_single_raw(300)  # 254
///
/// @export
#[extendr]
fn pass_single_raw(r: u8) -> u8 {
    passing_values::pass_single_raw_fn(r)
}

/// Left shift a raw vector in Rust
///
/// Demonstrates passing a raw (byte) vector back and forth between R and
/// Rust, left shifting the values in Rust to prove it works.
///
/// The vector passed to this function must be contain raw values, the
/// byte type in R. The Rust function signature is
/// `pass_multiple_raws_fn(r: &[u8]) -> Vec<u8>`. R does not support NAs for
/// raw types. Values should be passed as a raw vector, from `as.raw()`.
///
/// @param r A raw vector to treat as bytes in Rust
/// @return bitwShiftL(r, 1)
///
/// @examples
/// pass_multiple_raws(as.raw(c(2, 4, 8)))  # c(4, 8, 16)
///
/// @export
#[extendr]
fn pass_multiple_raws(r: &[u8]) -> Vec<u8> {
    passing_values::pass_multiple_raws_fn(r)
}

/// Uppercase a character value in Rust
///
/// Demonstrates passing a character (string) value back and forth between R and
/// Rust, uppercasing the string in Rust to prove it works.
///
/// The value passed to this function must be a character (string) or an
/// NA_character_. The Rust function signature is
/// `pass_single_character_fn(s: Option<String>) -> Option<String>`, with NA's
/// being handled as `None` in Rust.
///
/// @param s A character value to treat as a string in Rust
/// @return toupper(s)
///
/// @examples
/// pass_single_character("hello")        # "HELLO"
/// pass_single_character(NA_character_)  # NA_character_
///
/// @export
#[extendr]
fn pass_single_character(s: Option<String>) -> Option<String> {
    passing_values::pass_single_character_fn(s)
}

/// Uppercase a character vector in Rust
///
/// Demonstrates passing a character (string) vector back and forth between R
/// and Rust, uppercasing the string in Rust to prove it works.
///
/// The vector passed to this function must be contain character values. The Rust
/// function signature is `pass_multiple_characters_fn(s: Vec<String>) -> Vec<String>`.
/// extendr_api(v0.2.0) current does not support a method (I could find) for
/// passing character vectors with NA's, so NA handling will need te be managed
/// on the R side for now.
///
/// @param s A character vector to treat as strings in Rust
/// @return toupper(s)
///
/// @examples
/// pass_multiple_characters(c("hello", "world"))  # c("HELLO", "WORLD")
///
/// @export
#[extendr]
fn pass_multiple_characters(s: CharVec) -> CharVec {
    passing_values::pass_multiple_characters_fn(s)
}

/// Bubble Sort a vector of doubles
///
/// Demonstrates using Rust to perform a Bubble Sort on a vector of doubles
///
/// @param input A double vector to sort
/// @return a sorted vector of doubles
///
/// @examples bubble_sort(runif(1000))
///
/// @export
#[extendr]
fn bubble_sort(input: Real) -> Real {
    algos::bubble_sort_fn(input)
}

/// Pass an Arrow RecordBatch back and forth
///
/// Demonstrates a strategy for passing an Arrow RecordBatch from/to R
///
/// The majority of the work here is being done by the
/// [RecordBatches](crate::structs::RecordBatches) struct and associated
/// functions that transform a serialized RecordBatch (raw vector) into
/// RecordBatches and back to a serialized RecordBatch using the IPC
/// specification.
///
/// @param srb a raw vector representing the contents of an IPC buffer
/// @return RecordBatches to be serialized back to an IPC buffer
///
/// @examples
/// mtcars_rb <- arrow::record_batch(mtcars)
/// mtcars_out <- arrow::write_to_raw(mtcars_rb, "stream")
/// mtcars_in_buffer <- as.raw(pass_arrow_record_batch(mtcars_out))
/// mtcars_in <- arrow::read_ipc_stream(mtcars_in_buffer)
///
/// @export
#[extendr]
fn pass_arrow_record_batch(srb: RecordBatches) -> RecordBatches {
    srb
}

/// Perform a sample multithreaded operation
///
/// Demonstrates the performance of hand-coded multithreaded tasks in Rust
///
/// This function uses native multithreading fron the Rust standard library to
/// calculate the square root of each number in a slice of floats. The algorithm
/// is the "Babylonian Method" of calculating square roots.
///
/// @param f a double vector to calculate the square root of
/// @return a double vector of square roots
///
/// @export
#[extendr]
fn multithreaded_naive_sqrt(f: &[f64]) -> Vec<f64> {
    algos::multithreaded_naive_sqrt(&f)
}

/// Perform a sample multithreaded operation (rayon)
///
/// Demonstrates the performance of multithreaded tasks using rayon in Rust
///
/// This function uses the rayon crate to parallelize calculating the square
/// root of each number in a slice of floats. The algorithm
/// is the "Babylonian Method" of calculating square roots.
///
/// @param f a double vector to calculate the square root of
/// @return a double vector of square roots
///
/// @export
#[extendr]
fn rayon_naive_sqrt(f: &[f64]) -> Vec<f64> {
    algos::rayon_naive_sqrt(&f)
}

mod export {
    use super::*;

    // Macro to generate exports.
    // This ensures exported functions are registered with R.
    // See corresponding C code in `entrypoint.c`.
    extendr_module! {
        mod rustbind;
        fn pass_single_integer;
        fn pass_multiple_integers;
        fn pass_single_double;
        fn pass_multiple_doubles;
        fn pass_single_logical;
        fn pass_multiple_logicals;
        fn pass_single_raw;
        fn pass_multiple_raws;
        fn pass_single_character;
        fn pass_multiple_characters;
        fn bubble_sort;
        fn pass_arrow_record_batch;
        fn multithreaded_naive_sqrt;
        fn rayon_naive_sqrt;
    }
}
