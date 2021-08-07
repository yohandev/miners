use std::sync::mpsc::{ Sender, Receiver, channel };

use threadpool::ThreadPool;

/// A wrapper over a thread pool that executes arbitrary jobs that return `T`,
/// all concurrently and without blocking.
pub struct ThreadJobs<T> 
{
    /// Threads over which the jobs are executed
    pool: ThreadPool,
    /// Jobs clone this handle
    send: Sender<T>,
    /// Owner of `self` receives through this
    recv: Receiver<T>,
}

impl<T> ThreadJobs<T>
{
    /// Create a new thread-pool of workers with the given number of threads
    pub fn new(threads: usize) -> Self
    {
        let pool = ThreadPool::new(threads);
        let (send, recv) = channel();

        Self { pool, send, recv }
    }
}

impl<T: Send + 'static> ThreadJobs<T>
{
    /// Push a job to be eventually completed and retrieved via [pull]
    pub fn push(&self, f: impl (FnOnce() -> T) + Send + 'static)
    {
        let send = self.send.clone();
        self.pool.execute(move || send.send(f()).unwrap());
    }

    /// Pull the results of completed jobs sent via [push]
    pub fn pull(&self) -> impl Iterator<Item = T> + '_
    {
        self.recv.try_iter()
    }

    /// Blocks the current thread until all current jobs are completed,
    /// and returns their results
    pub fn join(&self) -> impl Iterator<Item = T> + '_
    {
        self.recv.iter()
    }
}