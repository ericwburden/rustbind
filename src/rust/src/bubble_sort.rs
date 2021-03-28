use extendr_api::prelude::{Real, RobjItertools};

/// Implementation of a Bubble Sort algorithm
pub(crate) fn bubble_sort_fn(input: Real) -> Real {
    let mut nvec: Vec<_> = input.collect();
    let len = nvec.len();

    for idx in 0..(len - 1) {
        let last_idx = len - idx - 1;
        for inner_idx in 0..last_idx {
            if nvec[inner_idx] > nvec[inner_idx + 1] {
                nvec.swap(inner_idx, inner_idx + 1)
            }
        }
    }

    nvec.iter().collect_robj().as_real_iter().unwrap()
}
