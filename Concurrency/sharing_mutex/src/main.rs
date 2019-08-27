use std::sync::{Mutex,Arc};
use std::thread;
//use std::rc::Rc;

fn main() {
    // Gonna use Arc, which from the book sounds dangerously similar
    // to a "semaphore" paired with a pointer
    //let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //let mut num = counter.lock().unwrap();
            //*num += 1
            *&mut *counter.lock().unwrap() += 1; // <- This cheeky one liner
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
