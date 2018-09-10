use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            let err = PoolCreationError {};
            return Err(err);
        }

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        let pool = ThreadPool { workers: workers };
        Ok(pool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {
            id: id,
            thread: thread,
        }
    }
}

#[derive(Debug)]
pub struct PoolCreationError {}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("thread pool creation error")
    }
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        "failed to create thread pool"
    }
}
