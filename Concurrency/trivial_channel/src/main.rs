use std::sync::mpsc;
use std::thread;
/// mpsc=> *multiple producer single consumer*
fn main() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        transmitter.send(val).unwrap(); // 
    });

    let received = receiver.recv().unwrap();
    // other way involves using `try_recv()`
    // => Result<Type T as Ok(), Error E as Err> 
    println!("Got {}", received);
}
