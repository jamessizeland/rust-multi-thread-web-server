use std::{
    sync::mpsc,
    thread::{spawn, JoinHandle},
};

/// Worker waits for orders and then executes them in its open thread
///
/// Instead of storing a vector of JoinHandle<()> instances in the thread pool, we’ll store instances of the Worker struct. Each Worker will store a single JoinHandle<()> instance. Then we’ll implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution. We’ll also give each worker an id so we can distinguish between the different workers in the pool when logging or debugging.
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = spawn(|| {}); // run when thread is created
        Worker { id, thread }
    }
}

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new threadpool of finite size
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // unrecoverable error if zero entered

        // initialize queue channels https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
        let (sender, reciever) = mpsc::channel();

        let mut workers = Vec::with_capacity(size); // like vec::new but preallocates space in the vector

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id)); // ids == index no.
        }

        ThreadPool { workers, sender }
    }

    /// Similar to thread::spawn but finite
    ///
    /// The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful in our situation: we need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
