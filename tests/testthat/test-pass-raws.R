test_that("Passing single raw values behaves as expected", {
  expect_equal(pass_single_raw(12), 24)
  
  # Single values passed as numbers 0-255, not from as.raw()
  expect_error(pass_single_raw(as.raw(12)))
  
  # Values less than 0 are coerced to 0
  expect_equal(pass_single_raw(-100), 0)
  
  # Values greater than 255 are coerced to 255
  expect_equal(pass_single_raw(300), 254)
  
  # Does not accept NAs
  expect_error(pass_single_raw(NA))
})

test_that("Passing multiple raws behaves as expected", {
  expect_equal(pass_multiple_raws(as.raw(c(2, 4, 8))), c(4, 8, 16))
  
  # Does not accept numeric vector
  expect_error(pass_multiple_raws(c(2, 4, 8)))
})