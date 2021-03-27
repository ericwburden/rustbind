# Currently only including R wrappers for passing integers and characters, since
# the Rust/extendr handling for doubles, logicals, and raws behaves basically as
# expected

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


# R Wrappers for passing characters --------------------------------------------

#' Provides `NA` handling for character vectors
#' 
#' The extendr API doesn't (as far as I can tell) support NA-handling in 
#' character vectors natively. This function emulates that support by replacing
#' elements in the output with `NA` where they are `NA` in the input.
#'
#' @param s A character vector to pass to Rust
#'
#' @return toupper(s)
#' @export
#'
#' @examples
#' pass_multiple_characters_wrapped(c("a", 7, NA))  # c("A", "7", NA)
pass_multiple_characters_wrapped <- function(s = c("a", "b", NA)) {
  s <- as.character(s)
  na_idx <- is.na(s)
  result <- pass_multiple_characters(replace(s, na_idx, ""))
  replace(result, na_idx, NA_character_)
}
