use std::thread;
use std::sync::mpsc;
/* Recall, the thread-safe smart pointers: 
 * to share ownership across multiple threads and allow the 
 * threads to allow them to mutate the value, we
 * need Arc<Mutex<T>>. The Arc type will let multiple 
 * workers own the receiver, and Mutex will ensure that only one worker gets
 * a job from the receiver at a time.
 */
use std::sync::{Arc, Mutex};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id_no in 0..size {
           // create threads and store them in `workers` 
           workers.push(Worker::new(id_no, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
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
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

