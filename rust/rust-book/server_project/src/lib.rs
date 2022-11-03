use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            match reciver.lock().expect("Error obtaining lock").recv() {
                Ok(job) => {
                    println!("Worker {id} got a job! Executing...");
                    job();
                    println!("Worker {id} execution finished.");
                },
                Err(_) => {
                    println!("Worker id {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Self { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }
        Self { workers, sender: Some(sender) }
    }

    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size > 0 {
            Ok(Self::new(size))
        } else {
            Err(PoolCreationError("Cannot run with 0 threads."))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            drop(self.sender.take());

            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
