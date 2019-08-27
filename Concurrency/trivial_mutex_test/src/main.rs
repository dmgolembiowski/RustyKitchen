use std::sync::Mutex;

fn main() {
    let m = Mutex::new(50);
    {
        *&mut *m.lock().unwrap() += 1;
    }

    println!("m = {:?}", m);
}
