use crate::structs::CharVec;
use crate::utils::flip;
use extendr_api::prelude::*;

/// Receives a single integer from R, multiplies it by 10, then returns it.
/// Demonstrates passing a single integer from/to R.
pub(crate) fn pass_single_integer_fn(n: Option<i32>) -> Option<i32> {
    n.map(|x| x * 10)
}

/// Receives an integer vector from R, multiplies it by 10, then returns it.
/// Demonstrates passing an integer vector from/to R.
pub(crate) fn pass_multiple_integers_fn(n: Int) -> Int {
    n.into_iter()
        .map(|x| x * 10)
        .collect_robj()
        .as_integer_iter()
        .unwrap()
}

/// Receives a double (float) value from R, multiplies it by 10, then returns it.
/// Demonstrates passing a single double from/to R.
pub(crate) fn pass_single_double_fn(f: Option<f64>) -> Option<f64> {
    f.map(|x| x * 10.0)
}

/// Receives a double (float) vector from R, multiplies it by 10, then returns it.
/// Demonstrates passing a double vector from/to R.
pub(crate) fn pass_multiple_doubles_fn(f: Real) -> Real {
    f.into_iter()
        .map(|x| x * 10.0)
        .collect_robj()
        .as_real_iter()
        .unwrap()
}

/// Receives a logical (bool) value from R, flips it, then returns it.
/// Demonstrates passing a single logical value from/to R.
pub(crate) fn pass_single_logical_fn(b: Option<bool>) -> Option<bool> {
    b.map(|x| !x)
}

/// Receives a logical (bool) vector from R, flips it, then returns it.
/// Demonstrates passing a logical vector from/to R.
pub(crate) fn pass_multiple_logicals_fn(b: Logical) -> Logical {
    b.into_iter()
        .map(flip)
        .collect_robj()
        .as_logical_iter()
        .unwrap()
}

/// Receives a raw (byte) value from R, shifts it left by 1 (wrapping), then returns it.
/// Demonstrates passing a single raw value from/to R.
pub(crate) fn pass_single_raw_fn(r: u8) -> u8 {
    r.wrapping_shl(1)
}

/// Receives a raw (byte) vector from R, shifts it left by 1 (wrapping), then returns it.
/// Demonstrates passing a raw vector from/to R.
pub(crate) fn pass_multiple_raws_fn(r: &[u8]) -> Vec<u8> {
    r.iter().map(|x| x.wrapping_shl(1)).collect()
}

/// Receives a string from R, uppercases it, then returns it
/// Demonstrates passing a single string value from/to R
pub(crate) fn pass_single_character_fn(s: Option<String>) -> Option<String> {
    s.map(|x| x.to_uppercase())
}

/// Receives a string vector from R, uppoercases it, then returns it
/// Demonstrates passing a string vector from/to R
pub(crate) fn pass_multiple_characters_fn(s: CharVec) -> CharVec {
    s.into_iter().map(|x| x.map(to_uppercase)).collect()
}

// Just a helper function
fn to_uppercase(s: String) -> String {
    s.to_uppercase()
}
