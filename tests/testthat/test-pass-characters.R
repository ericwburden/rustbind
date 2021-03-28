# Tests for extendr-wrappers/pass_single_character -----------------------------

test_that("Passing single character (string) behaves as expected", {
  expect_equal(pass_single_character("hello"), "HELLO")
  expect_equal(pass_single_character(NA_character_), NA_character_)
})


# Tests for extendr-wrappers/pass_multiple_characters --------------------------

test_that("Passing multiple characters (strings) behaves as expected", {
  expect_equal(pass_multiple_characters(c("hello", "world")), c("HELLO", "WORLD"))
  expect_equal(pass_multiple_characters(NA_character_), NA_character_)
  expect_equal(pass_multiple_characters(c("a", 7, NA)), c("A", "7", NA))
})
