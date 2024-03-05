#' @title A promise-based text file writer
#' @description This function writes a vector of lines
#'   to a text file asynchronously on another thread
#'   and returns a promise that resolves to the number of bytes written
#' @param content A vector of lines to write to the file
#' @param path The path to the file to write
#' @param append A logical value indicating whether to append to the file
#' @return A promise that resolves to the number of bytes written
#' @export
asyncWriteLines <- function(content, path, append = FALSE) {

  runtime <- getRuntime()

  # create a new task to read the file
  task <- runtime$async_write_lines_task(path, content, append = append)
  # start the task
  task$run()

  # create a promise that resolves when the task is done
  p <- promises::promise(function(resolve, reject) {

    # the interval to check the task state is 5ms
    poll_interval <- 0.05

    # check the task state
    check <- function() {
      # get the current state of the task
      state <- task$state()
      # switch on the state
      switch(
        state,
        pending = later::later(check, poll_interval),
        done = resolve(task$value()),
        error = reject(task$value())
      )
    }

    # Run the recursive check function
    check()

  })

  # return the promise
  return(p)

}
