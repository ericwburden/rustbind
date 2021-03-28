# This R script includes functions that 'wrap' the functions provided by extendr
# to extend the functionality on the R side, including checking for length-one
# vectors, coercing double vectors to integer vectors, and manually handling NA's

# R Wrappers for passing integers ----------------------------------------------

#' Provides better type handling for `pass_single_integer()`
#' 
#' Coerces input to integers and enforces single values before calling the
#' Rust implementation.
#'
#' @param n A numeric value to pass to Rust
#'
#' @return n * 10
#' @export
#'
#' @examples
#' pass_single_integer_wrapped(NA)  # NA_integer_
#' pass_single_integer_wrapped(5)   # 50
pass_single_integer_wrapped <- function(n = 5) {
  length(n) > 1 && stop("`pass_single_integer()` only accepts single values.")
  n <- as.integer(n)
  as.numeric(pass_single_integer(n))
}


#' Provides better type handling for `pass_multiple_integers()`
#' 
#' Coerces input values to integers and replaces output vector elements with NA
#' where appropriate.
#'
#' @param n A numeric vector to pass to Rust
#'
#' @return n * 10
#' @export
#'
#' @examples
#' pass_multiple_integers_wrapped(c(3, 5))   # c(30, 50)
#' pass_multiple_integers_wrapped(c(1, NA))  # c(10, NA)
pass_multiple_integers_wrapped <- function(n = c(5, 7, 9)) {
  n <- as.integer(n)
  result <- as.numeric(pass_multiple_integers(n))
  replace(result, is.na(n), NA)  # Handle NA's
}
