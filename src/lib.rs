use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{spawn, JoinHandle},
};

// https://doc.rust-lang.org/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
type Job = Box<dyn FnOnce() + Send + 'static>; // type aliases allow us to make long types shorter

enum Message {
    NewJob(Job),
    Terminate,
}

/// Worker waits for orders and then executes them in its open thread
///
/// Instead of storing a vector of JoinHandle<()> instances in the thread pool, we’ll store instances of the Worker struct. Each Worker will store a single JoinHandle<()> instance. Then we’ll implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution. We’ll also give each worker an id so we can distinguish between the different workers in the pool when logging or debugging.
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>, // https://doc.rust-lang.org/nightly/core/option/index.html
}

impl Worker {
    /// Create a new job worker with its own thread
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = spawn(move || // run infinitely when thread is created
            loop {
                // call lock to acquire the mutex, might fail if mutex is in a 'poisoned' state
                // which happens if another thread panics while holding the lock
                // this blocks so only one thread at a time is ever waiting for a job
            let message = receiver.lock().expect("receiver lock poisoned by another thread").recv().expect("thread holding sending side of channel has shutdown");
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job, executing.", id);
                    job(); // we can execute any function here, sent to this worker
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break; // off ramp out of this infinite loop
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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

        // initialize new async channel https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
        let (sender, receiver) = mpsc::channel();

        // Need thread-safe smart pointers to share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>. The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); // like vec::new but preallocates space in the vector

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver))); // ids == index no.
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
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            // Each worker will stop receiving requests on the channel once it gets a terminate message. So, we can be sure that if we send the same number of terminate messages as there are workers, each worker will receive a terminate message before join is called on its thread.
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // if Worker holds an Option<thread::JoinHandle<()>> instead, we can call the take method on the Option to move the value out of the Some variant and leave a None variant in its place, i.e. a Worker that is running will have a Some variant in thread, and when we want to clean up a Worker, we’ll replace Some with None so the Worker doesn’t have a thread to run.
            if let Some(thread) = worker.thread.take() {
                // remove Some thread and replace with None, do nothing otherwise
                thread.join().expect("failed to gracefully shutdown a thread");
            }
        }
    }
}
