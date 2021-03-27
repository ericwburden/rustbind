# Tests for extendr-wrappers/pass_single_character -----------------------------

test_that("Passing single character (string) behaves as expected", {
  expect_equal(pass_single_character("hello"), "HELLO")
  expect_equal(pass_single_character(NA_character_), NA_character_)
})

test_that("Passing multiple characters (strings) behaves as expected", {
  expect_equal(pass_multiple_characters(c("hello", "world")), c("HELLO", "WORLD"))
})


# Tests for extendr-wrappers/pass_multiple_characters --------------------------

test_that("Passing multiple characters displays known bugs", {
  # The current Rust implementation doesn't support NA values in character vectors
  # TODO: Submit a PR to fix this
  expect_error(pass_multiple_characters(c("hi", NA_character_)))
})


# Tests for R-wrappers/pass_multiple_characters_wrapped ------------------------

test_that("R-wrapped passing multiple characters doesn't introduce new bugs", {
  expect_equal(
    pass_multiple_characters_wrapped(c("hello", "world")), 
    c("HELLO", "WORLD")
  )
})

test_that("R-wrapped passing multiple characters corrects known issues", {
  expect_equal(pass_multiple_characters_wrapped(c("a", 7, NA)), c("A", "7", NA))
})
