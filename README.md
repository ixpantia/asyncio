# AsyncReader

As the name suggests, this a very simple async file reader and writer for
interacting with files in a non-blocking way from R. It runs the file reading
in a separate thread and uses
[promises](https://rstudio.github.io/promises/index.html) as an API to
eventually return the result of the operation.

## Installation


> **Note**: This package is not yet on CRAN and is still just an experiment.
> You will also need `rustc` and `cargo` installed in your system to compile the
> Rust code.

```r
# With remotes
remotes::install_github("ixpantia/asyncio")
# With pak
pak::pak("ixpantia/asyncio")
```

## Usage

This package works best when used inside a plumber API or any other async environment like
Shiny. Here is an example of how to use it in a Plumber API to asynchronously read and write
to a file.

```R
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
```

## Changing the number of worker threads

`asyncio` uses actual operating system threads, not additional R proccesses,
therefore spawning additional threads is not as expensive as it would be if
using the `future` package. You can change the number of worker threads by
setting the option `asyncio.threads` before calling any of the async functions.

By default, `asyncio.threads` is set to 4. You can change it like this:

```R
options(asyncio.threads = 10)
```

## Improvements

1. Add more file reading functions.
1. Add more options for the file reading functions.
1. Add other I/O operations like TCP, UDP, etc.
