mod async_read_lines;
mod async_write_lines;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use async_read_lines::AsyncReadLinesTask;
use async_write_lines::AsyncWriteLinesTask;
use extendr_api::prelude::*;

pub struct AsyncRuntime {
    tp: threadpool::ThreadPool,
}

// This enum will specify the state of the async read task.
// It will be used to communicate the state of the task to R.
pub enum AsyncTaskState<T> {
    Pending,
    Done(Option<T>),
    Error(String),
}

#[extendr]
impl AsyncRuntime {
    fn new(workers: i32) -> Self {
        let tp = threadpool::ThreadPool::new(workers as usize);
        AsyncRuntime { tp }
    }
    fn async_read_lines_task(&self, file_path: &str) -> AsyncReadLinesTask<'_> {
        AsyncReadLinesTask {
            tp: &self.tp,
            path: PathBuf::from(file_path),
            state: Arc::new(Mutex::new(AsyncTaskState::Pending)),
        }
    }
    fn async_write_lines_task(
        &self,
        file_path: &str,
        lines: Vec<String>,
        append: bool,
    ) -> AsyncWriteLinesTask<'_> {
        AsyncWriteLinesTask {
            tp: &self.tp,
            path: PathBuf::from(file_path),
            state: Arc::new(Mutex::new(AsyncTaskState::Pending)),
            file_contents: Some(lines),
            append,
        }
    }
}

#[extendr]
impl AsyncReadLinesTask<'_> {
    fn run(&self) {
        self.__run();
    }
    fn value(&self) -> Robj {
        self.__value()
    }
    fn state(&self) -> &str {
        self.__state()
    }
}

#[extendr]
impl AsyncWriteLinesTask<'_> {
    fn run(&mut self) {
        self.__run();
    }
    fn value(&self) -> Robj {
        self.__value()
    }
    fn state(&self) -> &str {
        self.__state()
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod asyncio;
    impl AsyncRuntime;
    impl AsyncReadLinesTask;
    impl AsyncWriteLinesTask;
}
