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

trait FnBox {
    fn call_box(self: Box<Self>);
}

/// Any `FnOnce()` closures can use this `call_box` method.
/// The implementation of `call_box` uses `(*self)()` to move
/// the closure out of the `Box<T>` and call the closure.
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// This implementation protects from the thing Raymond Hettinger
/// warned about in his concurrency talk from San Francisco Bay 2017 Con.
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// struct Job;
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
   
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    /*
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
    */
}

/// Using `if let` to destructure the `Some` and get the
/// thread; then we call `join` on the thread.
/// If a worker's thread is already `None`, we know that 
/// worker has already had its thread cleaned up, so
/// nothing happens.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

