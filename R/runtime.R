asyncEnv <- new.env(parent = emptyenv())

asyncEnv$runtime <- NULL

getRuntime <- function() {
  if (is.null(asyncEnv$runtime)) {
    warning("Creating a new AsyncRuntime with 4 threads")
    threads <- getOption("async.threads", default = 4)
    asyncEnv$runtime <- AsyncRuntime$new(threads)
  }
  return(asyncEnv$runtime)
}
