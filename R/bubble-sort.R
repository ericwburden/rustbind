#' Bubble Sorting - R Implementation
#'
#' @param nums numeric vector to sort
#' @return a sorted integer vector
#' @export
bubble_sort_r <- function(nums) {
  if (missing(nums)) { nums <- stats::runif(1000) }
  
  n <- length(nums)
  for (i in 1:(n-1)) {
    for (j in 1:(n - i)) {
      if (nums[j] > nums[j + 1]) {
        temp <- nums[j]
        nums[j] <- nums[j + 1]
        nums[j + 1] <- temp
      }
    }
  }
  
  nums
}