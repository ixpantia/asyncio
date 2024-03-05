asyncEnv <- new.env(parent = emptyenv())

asyncEnv$runtime <- NULL

getRuntime <- function() {
  if (is.null(asyncEnv$runtime)) {
    threads <- getOption("asyncio.threads", default = 4)
    cat(paste("Creating runtime with", threads, "threads\n"), file = stderr())
    asyncEnv$runtime <- AsyncRuntime$new(threads)
  }
  return(asyncEnv$runtime)
}
