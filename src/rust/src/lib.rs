use std::{
    io::BufRead,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use extendr_api::prelude::*;

// This enum will specify the state of the async read task.
// It will be used to communicate the state of the task to R.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AsyncReadState {
    Pending,
    Done,
    Error,
}

// This struct will be used to manage the async read task.
struct AsyncReadTask {
    // The path to the file to read.
    path: PathBuf,
    // The state of the async read task.
    state: Arc<Mutex<AsyncReadState>>,
    // The contents of the file read or the error
    // message if the read failed.
    file_contents: Arc<Mutex<Vec<String>>>,
}

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

#[extendr]
impl AsyncReadTask {
    fn new(file_path: &str) -> Self {
        AsyncReadTask {
            path: PathBuf::from(file_path),
            state: Arc::new(Mutex::new(AsyncReadState::Pending)),
            file_contents: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn run(&self) {
        let state = self.state.clone();
        let file_contents = self.file_contents.clone();
        let path = self.path.clone();
        std::thread::spawn(move || {
            let read_contents = read_file_lines(&path);
            match read_contents {
                Ok(contents) => {
                    *state.lock().unwrap() = AsyncReadState::Done;
                    *file_contents.lock().unwrap() = contents;
                }
                Err(fi) => {
                    *state.lock().unwrap() = AsyncReadState::Error;
                    *file_contents.lock().unwrap() = vec![fi.into()];
                }
            }
        });
    }
    fn value(&self) -> Vec<String> {
        let mut contents = self.file_contents.lock().unwrap();
        std::mem::take(&mut *contents)
    }
    fn state(&self) -> &str {
        match *self.state.lock().unwrap() {
            AsyncReadState::Pending => "pending",
            AsyncReadState::Done => "done",
            AsyncReadState::Error => "error",
        }
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod asyncreader;
    impl AsyncReadTask;
}
