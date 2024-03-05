use std::{
    io::BufRead,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use extendr_api::prelude::*;

use crate::AsyncTaskState;

fn read_file_lines(path: &Path) -> Result<Vec<String>> {
    // Open the file and read its contents.
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader
        // We separate the into the lines of the file.
        .lines()
        // We collect the lines into a vector of strings.
        .map(|l| {
            // If there is an error we map it to a string.
            // R does not support non character types as
            // error messages.
            l.map_err(|e| e.to_string().into())
        })
        .collect::<Result<Vec<String>>>()?;
    Ok(lines)
}

// This struct will be used to manage the async read task.
pub(crate) struct AsyncReadLinesTask<'a> {
    pub(crate) tp: &'a threadpool::ThreadPool,
    // The path to the file to read.
    pub(crate) path: PathBuf,
    // The state of the async read task.
    pub(crate) state: Arc<Mutex<AsyncTaskState<Vec<String>>>>,
}

impl AsyncReadLinesTask<'_> {
    pub(crate) fn __run(&self) {
        let state = self.state.clone();
        let path = self.path.clone();
        self.tp.execute(move || {
            let read_contents = read_file_lines(&path);
            match read_contents {
                Ok(contents) => {
                    *state.lock().unwrap() = AsyncTaskState::Done(Some(contents));
                }
                Err(fi) => {
                    *state.lock().unwrap() = AsyncTaskState::Error(fi.to_string());
                }
            }
        });
    }
    pub(crate) fn __value(&self) -> Robj {
        let mut state = self.state.lock().unwrap();
        match &mut *state {
            AsyncTaskState::Done(contents) => contents
                .take()
                .expect("Once done it should always be Some(_)")
                .into_robj(),
            AsyncTaskState::Error(e) => e.as_str().into_robj(),
            _ => NULL.into_robj(),
        }
    }
    pub(crate) fn __state(&self) -> &str {
        match *self.state.lock().unwrap() {
            AsyncTaskState::Pending => "pending",
            AsyncTaskState::Done(_) => "done",
            AsyncTaskState::Error(_) => "error",
        }
    }
}
