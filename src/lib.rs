pub struct ThreadPool;

impl ThreadPool {
    /// Create a new threadpool of finite size
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
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
