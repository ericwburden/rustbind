test_that("Rust implementation works", {
  input <- c(9, 8, 7, 6, 5)
  expected <- c(5, 6, 7, 8, 9)
  expect_identical(bubble_sort(input), expected)
})

test_that("R implementation works", {
  input <- c(9, 8, 7, 6, 5)
  expected <- c(5, 6, 7, 8, 9)
  expect_identical(bubble_sort_r(input), expected)
})