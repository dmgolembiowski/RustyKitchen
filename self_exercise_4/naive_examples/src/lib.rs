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


/// `Message` will be either a `NewJob` variant that holds
/// the `Job` the thread should run, or it will be a 
/// `Terminate` variant that will cause the thread to exit 
/// its loop and stop.
/// Therefore, `Message` will either need to extend the `Job` trait
/// or replace it entirely so that the channel will use its commands
/// for each of the producers.
enum Message {
    NewJob(Job),
    Terminate,
}



struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// This implementation protects from the thing Raymond Hettinger
/// warned about in his concurrency talk from San Francisco Bay 2017 Con.
impl Worker {
    fn new(id: usize,
           receiver: Arc<Mutex<mpsc::Receiver<Message>>>)
        -> Worker 
    {
    //fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                //let job = receiver.lock().unwrap().recv().unwrap();
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    },

                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    //sender: mpsc::Sender<Job>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the numer of threads in the pool.
    /// 
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
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
            self.sender.send(Message::NewJob(job)).unwrap();
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
    /// `ThreadPool::drop` iterates over `&mut self.workers` twice
    /// to serially call `thread.join()`  on each worker's thread.
    /// If the program tried to send a message and `join` immediately
    /// in the same loop, the function could not guarantee that the
    /// worker in the current iteration would be the one to get the 
    /// message from the channel.
    /// 
    /// (This is a safeguard against deadlock!)
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

