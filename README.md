# Minimal Example of Calling Rust from R

[![R build status](https://github.com/ericwburden/rustbind/workflows/R-CMD-check/badge.svg)](https://github.com/ericwburden/rustbind/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a template package to demonstrate how to call Rust from R using the [extendr-api](https://crates.io/crates/extendr-api) crate, based on the [helloextendr](https://github.com/extendr/helloextendr) package.


## Installation

Before you can develop with this package, you need to install a working Rust toolchain. It's probably best to use [rustup.](https://rustup.rs/)

On Windows, you'll also have to add the `i686-pc-windows-gnu` and `x86_64-pc-windows-gnu` targets:
```
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
```

Since this project is intended to provide reference implementations, you will probably find more utility in cloning the repository than installing the package. If you insist, you can install rustbind with:

``` r
devtools::install_github("ericwburden/rustbind")
```

## Development

To build this package, and generate the Rust function wrappers in `R/extendr-wrappers.R`, you should prefer to source the `build.R` build script over the 'Clean and Rebuild' menu option in RStudio.

## Usage

In order to demonstrate the process of adding a simple Rust function, callable by R, via the `extendr_api` crate, consider the [Bubble Sort](https://www.geeksforgeeks.org/bubble-sort/) algorithm. Briefly, Bubble Sorting requires traversing a list of elements and exchanging them pair-wise until the entire list is in order, as demonstrated in the below pseudocode. 

```
function bubble_sort(arr) {
    n = len(arr) 
  
    // Traverse through all array elements 
    for i in range from 0 to n {
        for j in range from 0 to (n-i-1) { 
        
          // For each element `j`, swap with element `j+1` if element `j` is
          // larger
          if arr[j] > arr[j+1] : 
            swap(arr[j], arr[j+1])
    }
  }
}
```

This is *not* an efficient way to sort a list (or vector), but it is an algorithm that can be run much faster by implementing it in Rust instead of R alone. 

### Adding the Rust Implementation

To keep the code nicely separated, start by creating a file `rustbind/src/rust/src/bubble_sort.rs` with the following code:

```rust
use extendr_api::prelude::{Real, RobjItertools};

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
```

In order to generate the wrappers and bindings, add the following to `rustbind/src/rust/src/lib.rs`:

```rust
use extendr_api::prelude::*;

mod bubble_sort;
// Other modules here...

/// Bubble Sort a vector of doubles
/// 
/// Demonstrates using Rust to perform a Bubble Sort on a vector of doubles
/// 
/// @params input A double vector to sort
/// @return a sorted vector of doubles
/// 
/// @examples bubble_sort(runif(1000))
/// 
/// @export
#[extendr]
fn bubble_sort(input: Real) -> Real {
    bubble_sort::bubble_sort_fn(input)
}

extendr_module! {
    mod rustbind;
    // Other functions to export go here...
    fn bubble_sort;
}
```

And that's it! After sourcing `build.R` in order to build, install, and load your package, you will be able to call `yourpackagename::bubble_sort(n)` and take advantage of Rust's blazing speed in your R code. Also, note the format of that doc comment. If you've written R packages before, you'll recognize those as [`roxygen2`](https://cran.r-project.org/web/packages/roxygen2/vignettes/roxygen2.html) comments, which are used to build the documentation for your package, as well as control whether or not the function is available outside your package (via package_name::function_name) through the '@export' tag.

### Was it Worth It?

Now, that's definitely a *relatively* more complicated way to just write Rust code instead of R code, was it worth it? I would argue that for any type of exploratory or *ad hoc* analysis, the answer is most definitely **no**. But, if you're writing R code that will be used in production to run the same type of algorithm over and over again, the speed gains are tremendous. Consider the analogous R implementation of the Bubble Sort algorithm:

```r
#' Bubble Sorting - R Implementation
#'
#' @param nums numeric vector to sort
#' @return a sorted integer vector
#' @export
bubble_sort_r <- function(nums) {
  nums <- if (missing(nums)) { stats::runif(1000) }
  
  n <- length(nums)
  for (i in 1:(n-1)) {
    for (j in 1:(n - i)) {
      if (nums[j] > nums[j + 1]) {
        temp <- nums[j]
        nums[j] <- nums[j + 1]
        nums[j + 1] <- temp
      }
    }
  }
  
  nums
}
```

Notwithstanding the slight modification needed because R doesn't provide a convenient way to swap values without using a temporary variable, this is nearly identical to our Rust implementation. But, if we benchmark the two functions...

```
                  test replications elapsed relative user.self sys.self user.child sys.child
2 bubble_sort_r(input)          100 384.256   24.789   384.045        0      0.022     0.123
1   bubble_sort(input)          100  15.501    1.000    15.502        0      0.000     0.000
```

We see that, for vectors of 10k random numbers, the Rust implementation is nearly 25x faster than the R implementation, for the same task. That's a pretty massive speedup, considering we've used a fairly naive implementation of this algorithm (we're copying the vector at least twice). So, there will definitely be situations where you will save a huge amount of processing time by implementing functions in Rust (just like if you were implementing an underlying function in C or C++), with the added safety guarantees of Rust. You may find you are even able to perform calculations that simply aren't feasible (at least not in any reasonable amount of time) in pure R. So, happy coding!

## Extending `extendr`

You may encounter situations in which `extendr` does not behave as expected or support your use case (yet). As of 2021-03-28, `extendr` v0.2.0 doesn't support (so far as I can tell) passing in character vectors that may contain NA's or correctly giving back integer vectors with NA's (they get converted to 0). There are at least two different ways to address these issues as they arise:

### Wrapping Rust Calls in R

One strategy is to 'wrap' the functions automatically generated by `extendr` to address these issues by modifying the input or output on the R side. For an example of this, see [`R/r-wrappers.R`](R/r-wrappers.R).

### Implementing R <-> Rust Conversions in Rust

Another strategy is to create structs and types in Rust with appropriate trait implementations (particularly `FromRobj` and `From<T> for Robj`) in your Rust module. This has the added benefit of plugging directly into the `extendr` infrastructure. For an example of this, see [`src/rust/src/structs/char_vec.rs`](src/rust/src/structs/char_vec.rs).
