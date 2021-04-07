test_that("Data frames can be successfully passed to/from Rust", {
  mtcars_rb <- arrow::record_batch(mtcars)
  mtcars_out <- arrow::write_to_raw(mtcars_rb, "stream")
  mtcars_in_buffer <- as.raw(pass_arrow_record_batch(mtcars_out))
  mtcars_in <- as.data.frame(arrow::read_ipc_stream(mtcars_in_buffer))
  
  # Note, the row names are lost when passing back and forth
  expect_equal(mtcars$mpg,  mtcars_in$mpg)
  expect_equal(mtcars$cyl,  mtcars_in$cyl)
  expect_equal(mtcars$disp, mtcars_in$disp)
  expect_equal(mtcars$hp,   mtcars_in$hp)
  expect_equal(mtcars$drat, mtcars_in$drat)
  expect_equal(mtcars$wt,   mtcars_in$wt)
  expect_equal(mtcars$qsec, mtcars_in$qsec)
  expect_equal(mtcars$vs,   mtcars_in$vs)
  expect_equal(mtcars$am,   mtcars_in$am)
  expect_equal(mtcars$gear, mtcars_in$gear)
  expect_equal(mtcars$carb, mtcars_in$carb)
})
