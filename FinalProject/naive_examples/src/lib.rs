pub struct ThreadPool;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
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

