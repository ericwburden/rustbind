# Tests for extendr-wrappers/pass_single_integer -------------------------------

test_that("Passing single integers behaves as expected", {
  int <- as.integer(5)
  
  expect_equal(pass_single_integer(int), 50)
  expect_identical(pass_single_integer(int), as.integer(50))
  
  # Correctly handle NA values
  expect_equal(pass_single_integer(NA_integer_), NA_integer_)
  
  # R numbers default to doubles, not integers
  expect_error(pass_single_integer(5))
})


# Tests for extendr-wrappers/pass_multiple_integers ----------------------------

test_that("Passing multiple integers behaves as expected", {
  ints <- as.integer(c(5, 7, 9))
  
  expect_equal(pass_multiple_integers(ints), c(50, 70, 90))
  expect_identical(pass_multiple_integers(ints), as.integer(c(50, 70, 90)))
  
  # pass_multiple_integers should accept a length-one vector
  expect_equal(pass_multiple_integers(as.integer(5)), 50)
  
  # R numeric vectors default to doubles, not integers
  expect_error(pass_multiple_integers(5))
  expect_error(pass_multiple_integers(c(5, 10)))
})

test_that("Passing multiple integers displays known bugs", {
  # TODO: Submit a PR to extender-api for these known bugs
  ints_na <- as.integer(c(5, NA))
  
  # We would expect to get a NA back, but we get 0 instead
  expect_equal(pass_multiple_integers(NA_integer_), 0)
  
  # pass_multiple_integers should return NA's in longer vectors, we get 0's 
  # in their place instead
  expect_equal(pass_multiple_integers(ints_na), c(50, 0))
})


# Tests for r-wrappers/pass_single_integer_wrapped -----------------------------

test_that("R-wrapped passing single integers doesn't introduce new bugs", {
  int <- as.integer(5)
  
  expect_equal(pass_single_integer_wrapped(int), 50)
  expect_equal(pass_single_integer_wrapped(NA_integer_), NA_integer_)
})

test_that("R-wrapped passing single integers corrects known issues", {
  # Attempts to coerce non-integer arguments to integers
  expect_equal(pass_single_integer_wrapped(5), 50)
  expect_equal(pass_single_integer_wrapped(NA), NA_integer_)
  expect_error(pass_single_integer_wrapperd(c(1, 2)))
  
  # Coercing strings to integers produces NA's
  expect_warning(a <- pass_single_integer_wrapped("no"), "NAs introduced by coercion")
  expect_equal(a, NA_integer_)
})


# Tests for r-wrappers/pass_multiple_integers_wrapped --------------------------

test_that("R-wrapped passing multiple integers doesn't introduce new bugs", {
  ints <- as.integer(c(5, 7, 9))
  expect_equal(pass_multiple_integers_wrapped(ints), c(50, 70, 90))
  expect_equal(pass_multiple_integers_wrapped(as.integer(5)), 50)
})

test_that("R-wrapped passing multiple integers corrects known issues", {
  expect_equal(pass_multiple_integers_wrapped(c(5, 7, 9)), c(50, 70, 90))
  expect_equal(pass_multiple_integers_wrapped(c(5, NA)), c(50, NA))
})


