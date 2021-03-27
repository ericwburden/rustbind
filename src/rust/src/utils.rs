use extendr_api::prelude::*;

pub(crate) fn flip(b: Bool) -> Bool {
    if b.is_true() {
        Bool::from(false)
    } else if b.is_false() {
        Bool::from(true)
    } else {
        b
    }
}
