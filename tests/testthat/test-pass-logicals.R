test_that("Passing single logicals behaves as expected", {
  expect_equal(pass_single_logical(TRUE), FALSE)
  expect_equal(pass_single_logical(FALSE), TRUE)
  expect_equal(pass_single_logical(NA), NA)
})

test_that("Passing multiple doubles behaves as expected", {
  expect_equal(pass_multiple_logicals(c(TRUE, FALSE, NA)), c(FALSE, TRUE, NA))
})