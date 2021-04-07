test_that("All four sqrt functions return the same result", {
  test_data <- runif(100)*10000
  standard <- sqrt(test_data)
  
  expect_equal(standard, sapply_naive_sqrt(test_data))
  expect_equal(standard, future_apply_naive_sqrt(test_data))
  expect_equal(standard, multithreaded_naive_sqrt(test_data))
  expect_equal(standard, rayon_naive_sqrt(test_data))
})
