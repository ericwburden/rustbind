test_that("Passing single doubles behaves as expected", {
  expect_equal(pass_single_double(5.5), 55)
  expect_equal(pass_single_double(NA_real_), NA_real_)
  expect_equal(pass_single_double(NaN), NaN)
  expect_equal(pass_single_double(Inf), Inf)
  expect_equal(pass_single_double(-Inf), -Inf)
})

test_that("Passing multiple doubles behaves as expected", {
  expect_equal(pass_multiple_doubles(5.5), 55)
  expect_equal(pass_multiple_doubles(NA_real_), NA_real_)
  expect_equal(pass_multiple_doubles(NaN), NaN)
  expect_equal(pass_multiple_doubles(Inf), Inf)
  expect_equal(pass_multiple_doubles(-Inf), -Inf)
  expect_equal(
    pass_multiple_doubles(c(5, 5.5, 5.55, NA, NaN, Inf, -Inf)), 
    c(50, 55, 55.5, NA, NaN, Inf, -Inf)
  )
})