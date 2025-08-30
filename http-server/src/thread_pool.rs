use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// A type alias for our "Job" type. As we discussed, this is a heap-allocated,
// thread-safe, and self-contained closure that can be executed once.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    // The workers vector will hold the threads that are waiting to execute jobs.
    workers: Vec<Worker>,
    // The sender is the way we will send Jobs from the ThreadPool to the Workers.
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // It doesn't make sense to have a thread pool with no threads.
        assert!(size > 0);

        // Create a new channel. The channel is the core communication primitive.
        // `sender` sends jobs, `receiver` receives them.
        let (sender, receiver) = mpsc::channel();

        // The receiver needs to be shared among multiple worker threads, and the workers
        // will need to mutate the receiver to get jobs from it.
        // To do this safely, we use Arc<Mutex<T>>.
        // 1. `Arc<T>`: Atomic Reference Counted pointer. It lets multiple owners hold
        //    immutable access to the same data. When the last owner is gone, the data is cleaned up.
        // 2. `Mutex<T>`: Mutual Exclusion primitive. It ensures that only one thread can
        //    access the data (the receiver) at any given time, preventing race conditions.
        let receiver = Arc::new(Mutex::new(receiver));

        // Pre-allocate space for our workers.
        let mut workers = Vec::with_capacity(size);

        // Create the specified number of worker threads.
        for id in 0..size {
            // We clone the Arc for each worker. This increases the reference count,
            // so the receiver will stay alive as long as at least one worker exists.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Executes a new job in the thread pool.
    ///
    /// This function takes a closure and sends it to an idle thread for execution.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Create a new job by putting the closure on the heap.
        let job = Box::new(f);
        // Send the job down the channel to the workers.
        // `send` returns a `Result`, but we `unwrap` because the only time it can fail
        // is if the receiver has been dropped. In our design, that means the pool is
        // shutting down, and we can't send new jobs anyway.
        self.sender.send(job).unwrap();
    }
}

// When the ThreadPool goes out of scope, we need to clean up gracefully.
// The `Drop` trait is Rust's equivalent of a destructor.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down. Waiting for all workers to finish.");

        // By dropping the sender, we close the channel. This will cause the
        // `receiver.lock().unwrap().recv()` call in the worker threads to return
        // an `Err`. This is the signal for the workers to break their loop and exit.
        drop(&self.sender);

        // Now we iterate over our workers and join each one.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // `take()` is used on the `Option<thread::JoinHandle<()>>` to move the
            // handle out of the worker struct, leaving `None` in its place.
            // We need to do this because `join()` consumes the handle.
            if let Some(thread) = worker.thread.take() {
                // `join()` will block the current thread (the main thread in this case)
                // until the worker's thread has finished its execution. This ensures
                // that we don't exit the program while jobs are still running.
                thread.join().unwrap();
            }
        }

        println!("All workers have been shut down.");
    }
}

// The Worker struct is an internal implementation detail.
struct Worker {
    id: usize,
    // Each worker has its own thread. The `JoinHandle` allows us to wait for the
    // thread to finish. It's wrapped in an `Option` so we can `take()` it during shutdown.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new Worker.
    ///
    /// The worker is a spawned thread that continuously waits for jobs on the receiver.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // The core worker loop.
                // 1. `receiver.lock().unwrap()`: Acquire the mutex lock. This blocks until the
                //    lock is available. `unwrap()` panics if the mutex was "poisoned" (a thread
                //    panicked while holding the lock).
                // 2. `.recv()`: Receive a job from the channel. This is a blocking call; the
                //    thread will sleep here until a job is available or the channel is closed.
                let job_result = receiver.lock().unwrap().recv();

                match job_result {
                    Ok(job) => {
                        // If we successfully received a job, execute it.
                        println!("Worker {} got a job; executing.", id);
                        job(); // This calls the `FnOnce` closure.
                    }
                    Err(_) => {
                        // If `recv()` returns an error, it means the sender has been dropped
                        // and no more jobs will be sent. The worker can exit its loop.
                        println!("Worker {} disconnecting; channel closed.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
