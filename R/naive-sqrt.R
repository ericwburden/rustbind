#' R version of the Bablyonian Method for calculating square roots
#'
#' @param n a double value (vector length 1)
#' @return the square root of n
naive_sqrt <- function(n) {
  if (length(n) > 1) { stop("Vectors longer than one not supported") }
  current_guess <- n
  adjustment <- 1
  error <- 0.000001
  while (current_guess - adjustment > error) {
    current_guess <- (current_guess + adjustment) / 2
    adjustment <- n / current_guess
  }
  current_guess
}

#' Apply naive_sqrt() to each item in a vector
#' 
#' Uses a base-R apply function to apply the naive_sqrt() algorithm to each
#' element in turn. Not parallel.
#'
#' @param n a double value (vector length 1)
#' @return the square roots of elements in n
#' @export
sapply_naive_sqrt <- function(n) {
  sapply(n, naive_sqrt)
}


#' Apply naive_sqrt() to each item in a vector
#' 
#' Uses `future.apply::future_sapply()`` function to apply the naive_sqrt() 
#' algorithm to the elements of n, in parallel.
#'
#' @param n a double value (vector length 1)
#' @return the square roots of elements in n
#' @export
future_apply_naive_sqrt <- function(n) {
  future.apply::future_sapply(n, naive_sqrt)
}