use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use extendr_api::prelude::*;

use crate::AsyncTaskState;

fn write_file_lines(path: &Path, content: Vec<String>, append: bool) -> Result<usize> {
    let file = std::fs::File::options()
        .write(true)
        .create(true)
        .append(append)
        .open(path)
        .map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);
    let mut written_bytes = 0;
    for line in content {
        written_bytes += writer.write(line.as_bytes()).map_err(|e| e.to_string())?;
        written_bytes += writer.write(b"\n").map_err(|e| e.to_string())?;
    }
    writer.flush().map_err(|e| e.to_string())?;
    Ok(written_bytes)
}

// This struct will be used to manage the async read task.
pub(crate) struct AsyncWriteLinesTask<'a> {
    pub(crate) tp: &'a threadpool::ThreadPool,
    // The path to the file to read.
    pub(crate) path: PathBuf,
    // The state of the async read task.
    pub(crate) state: Arc<Mutex<AsyncTaskState<usize>>>,
    // The contents of the file read or the error
    // message if the read failed.
    pub(crate) file_contents: Option<Vec<String>>,
    // Whether to append to the file or not.
    pub(crate) append: bool,
}

impl AsyncWriteLinesTask<'_> {
    pub(crate) fn __run(&mut self) {
        let state = self.state.clone();
        // Take the value inside the file_contents option and replace it with None.
        let file_contents = self.file_contents.take();
        let append = self.append;
        let path = self.path.clone();
        self.tp.execute(move || {
            if let Some(file_contents) = file_contents {
                let written_bytes = write_file_lines(&path, file_contents, append);
                match written_bytes {
                    Ok(contents) => {
                        *state.lock().unwrap() = AsyncTaskState::Done(contents);
                    }
                    Err(fi) => {
                        *state.lock().unwrap() = AsyncTaskState::Error(fi.to_string());
                    }
                }
            }
        });
    }
    pub(crate) fn __value(&self) -> Robj {
        let state = self.state.lock().unwrap();
        match &*state {
            AsyncTaskState::Done(contents) => contents.into_robj(),
            AsyncTaskState::Error(e) => e.into_robj(),
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
