use std::thread;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    drop(v); // use of moved `v` used here after drop

    handle.join().unwrap();
}
