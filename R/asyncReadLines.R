#' @title A promise-based text file reader
#' @description This function reads a text file asynchronously
#'   on another thread and returns a promise that resolves to an array
#'   of lines from the file.
#' @param file_path The path to the file to read
#' @return A promise that resolves to an array of lines from the file
#' @export
asyncReadLines <- function(file_path) {

  runtime <- getRuntime()

  # create a new task to read the file
  task <- runtime$async_read_lines_task(file_path)
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
