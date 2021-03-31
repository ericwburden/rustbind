if ("rustbind" %in% .packages()) devtools::unload("rustbind")
system2("R", args = c("CMD", "INSTALL", "--preclean", "--no-multiarch", "--with-keep.source", "--clean", "."))
devtools::install(build_vignettes = TRUE)

library(rustbind)
target_file <- rprojroot::is_r_package$find_file("R/extendr-wrappers.R")
cat(
  .Call("wrap__make_rustbind_wrappers", use_symbols = TRUE, package_name = "rustbind"),
  file = target_file
)
message("Rewrote 'R/extendr-wrappers.R'")

rm(list = ls())

devtools::document()
devtools::check_built(".")
library(rustbind)