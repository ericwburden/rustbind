% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/naive-sqrt.R
\name{future_apply_naive_sqrt}
\alias{future_apply_naive_sqrt}
\title{Apply naive_sqrt() to each item in a vector}
\usage{
future_apply_naive_sqrt(n)
}
\arguments{
\item{n}{a double value (vector length 1)}
}
\value{
the square roots of elements in n
}
\description{
Uses `future.apply::future_sapply()`` function to apply the naive_sqrt()
algorithm to the elements of n, in parallel.
}
