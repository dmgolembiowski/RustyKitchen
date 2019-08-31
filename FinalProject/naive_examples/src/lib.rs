use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
           // create threads and store them in the vector 
        }

        ThreadPool {
            threads 
        }
    }
    // Define the `execute` method on `ThreadPool` to take
    // a closure as a parameter (as Fn, FnMut, or FnOnce)
    
    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
        // Using `()` after `FnOnce` b/c this parameter 
        // represents a closure that takes no parameters
        // and doesn't return a value.
        {
            
        }
}

