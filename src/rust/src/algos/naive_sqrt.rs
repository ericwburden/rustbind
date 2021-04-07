use extendr_api::prelude::*;
use rayon::prelude::*;

/// Implements the [Babylonian Method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
/// for calculating square root of a float
#[allow(dead_code)]
pub(crate) fn naive_sqrt(n: &f64) -> f64 {
    let mut current_guess = *n;
    let mut adjustment = 1.0;
    let error = 0.000001;
    while current_guess - adjustment > error {
        current_guess = (current_guess + adjustment) / 2.0;
        adjustment = n / current_guess;
    }
    current_guess
}

/// Demonstrates the manual strategy for multi-threading, rolling the
/// multi-threaded code by hand.
#[allow(dead_code)]
pub(crate) fn multithreaded_naive_sqrt(floats: &[f64]) -> Vec<f64> {
    let thread_count = match floats.len() {
        0 => return Vec::new(),
        1..=1_000 => 1,
        1_001..=10_000_000 => floats.len() / 1_000,
        _ => 10_000,
    };
    let mut threads = Vec::with_capacity(thread_count);
    let chunk_size = (floats.len() / thread_count) + 1;

    for chunk in floats.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let thread = std::thread::spawn(move || {
            let sqrts: Vec<_> = chunk.iter().map(naive_sqrt).collect();
            sqrts
        });
        threads.push(thread);
    }

    let mut output: Vec<f64> = Vec::with_capacity(floats.len());
    for thread in threads {
        match thread.join() {
            Ok(squared_floats) => output.extend(&squared_floats),
            Err(e) => std::panic::resume_unwind(e),
        }
    }

    output
}

/// Demonstrates using [rayon](https://github.com/rayon-rs/rayon) to parallelize
/// operations (much nicer looking, isn't it?)
#[allow(dead_code)]
pub(crate) fn rayon_naive_sqrt(floats: &[f64]) -> Vec<f64> {
    floats.par_iter().map(naive_sqrt).collect()
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[allow(dead_code)]
    fn assert_floats_match(a: f64, b: f64) {
        let acceptable_error = 0.000001;
        let diff = (a - b).abs();
        if diff > acceptable_error {
            panic!("Expected {}, found {}.", a, b);
        }
    }

    #[test]
    fn test_square_root_one_number() {
        let input = 256.5;
        let expected = 16.015617378047367;
        assert_floats_match(naive_sqrt(&input), expected);
    }

    #[test]
    fn test_square_root_multithreaded() {
        let inputs: Vec<f64> = vec![599.5; 10_000];
        let expected = 24.484689133507388;
        let result = multithreaded_naive_sqrt(&inputs);
        assert_eq!(inputs.len(), result.len());
        for f in result.iter() {
            assert_floats_match(*f, expected);
        }
    }

    #[test]
    fn test_square_root_rayon() {
        let inputs: Vec<f64> = vec![1048.1; 10_000];
        let expected = 32.374372580794144;
        let result = rayon_naive_sqrt(&inputs);
        assert_eq!(inputs.len(), result.len());
        for f in result.iter() {
            assert_floats_match(*f, expected);
        }
    }
}
