//! This module demonstrates implementing a new struct that can be passed in and
//! out of Rust functions annotated with #[extendr]. As of the writing of this
//! module, extendr_api (v0.2.0) does not support passing in/out a character
//! vector that may contain NA's by default. To allow for this, I've implemented
//! [FromRobj](extendr__api::robj::FromRobj) for a newtype
//! [CharVec](crate::char_ved::CharVec), which wrap a Vec<Option<String>> where
//! NA's are represented by `None`. I've also implemented `From<CharVec>` for
//! `Robj`, allowing extendr to cast a `CharVec` to an `Robj` representing a
//! character vector.

use extendr_api::prelude::{na_str, FromRobj, Robj};
use std::iter::FromIterator;

/// Rust type that wraps a Vec<Option<String>>, where `None` represents an
/// NA_character_ from R
pub struct CharVec(Vec<Option<String>>);

/// For converting `CharVec` to `Robj`
impl From<CharVec> for Robj {
    fn from(cv: CharVec) -> Robj {
        Robj::from(cv.0)
    }
}

/// For converting `Robj` to `CharVec`
impl<'a> FromRobj<'a> for CharVec {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_na() {
            // A single NA_character_ is represented by a Vec containing a single None
            Ok(CharVec(vec![None]))
        } else if let Some(v) = robj.as_string_vector() {
            // Character values to Some(value), NA's to None
            let char_vec: Vec<_> = v.into_iter().map(parse_rchar).collect();
            Ok(CharVec(char_vec))
        } else {
            Err("Input must be a character vector.")
        }
    }
}

// Helper function
#[rustfmt::ignore]
pub(crate) fn parse_rchar(rchar: String) -> Option<String> {
    if rchar == na_str() { None } else { Some(rchar) }
}

/// Allow for iterating over the Vec<Option<String>> wrapped by CharVec
impl IntoIterator for CharVec {
    type Item = Option<String>;
    type IntoIter = std::vec::IntoIter<Option<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Allows for collecting a CharVec from any iterator that yields <Option<String>>
impl FromIterator<Option<String>> for CharVec {
    fn from_iter<I: IntoIterator<Item = Option<String>>>(iter: I) -> Self {
        CharVec(iter.into_iter().collect())
    }
}
