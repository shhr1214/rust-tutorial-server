use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            let err = PoolCreationError {};
            return Err(err);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        let pool = ThreadPool {
            workers: workers,
            sender: sender,
        };
        Ok(pool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate message to all workers.");

        // worker の数だけ terminate を送っている。
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutting down all workers.");

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} get a job; executing.", id);

                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
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
