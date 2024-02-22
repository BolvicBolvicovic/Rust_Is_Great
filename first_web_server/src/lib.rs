use std::thread;

pub struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,	
}

impl Worker {
	pub fn new(size: usize) -> Worker {
		Worker {id: size, thread: thread::spawn(||{})}
	}
}

pub struct ThreadPool {
	threads: Vec<Worker>,
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
		assert!(size > 0);
		let mut threads = Vec::with_capacity(size);
		for id in 0..size {
			threads.push(Worker::new(id));
		}
		ThreadPool{threads}
	}
	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
	{
		
	}
}
