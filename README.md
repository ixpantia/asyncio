# AsyncReader

As the name suggests, this a very simple async reader for reading files in a non-blocking
way from R. It runs the file reading in a separate thread and uses [promises](https://rstudio.github.io/promises/index.html)
as an API to eventually return the file contents.

## Installation


> **Note**: This package is not yet on CRAN and is still just an experiment.
> You will also need `rustc` and `cargo` installed in your system to compile the
> Rust code.

```r
pak::pak("andyquinterom/asyncreader")
```

## Usage

```r
library(asyncreader)
library(promises)

# Read a file
asyncReadLines("path/to/file.txt") %...>%
  print()
```

## Improvements

1. Use a thread pool rather than creating a new thread for each file read.
2. Add more file reading functions.
3. Add more options for the file reading functions.
