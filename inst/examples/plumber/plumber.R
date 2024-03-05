library(asyncio)
library(promises)

options(asyncio.threads = 10)
temp_file <- tempfile()

#* @post /write
#* @param line The line to write to the file
#* @parser text
function(line) {
  asyncWriteLines(line, temp_file, append = TRUE) %...>% {
    paste("You wrote:", ., "bytes to the file")
  } %...!% {
    paste("Failed to write to the file:", .)
  }
}

#* @get /read
function() {
  asyncReadLines(temp_file) %...!% {
    paste("Failed to read from the file, try writing something first:", .)
  }
}
