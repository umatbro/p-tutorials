use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
pub struct PoolCreationError(pub &'static str);

impl ThreadPool {
    ///Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }
        Self { threads }
    }

    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size > 0 {
            Ok(Self::new(size))
        } else {
            Err(PoolCreationError("Cannot run with 0 threads."))
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
    }
}