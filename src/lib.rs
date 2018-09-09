use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            let err = PoolCreationError {};
            return Err(err);
        }

        let pool = ThreadPool {};
        Ok(pool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
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
