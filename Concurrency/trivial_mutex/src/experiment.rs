use std::sync::Mutex;

fn main() {
    let m = Mutex::new(50);

    {
        let mut num = m.lock().unwrap();
        //          = LockResult<MutexGuard<T>>.unwrap();
        //          = MutexGuard<T>; //such that there exists
        //          num::Deref (...) -> num::......????

        //             &mut *m.lock().unwrap() += 1;
        //
   }
    *&mut *m.lock().unwrap() += 1;
    println!("m = {:?}", m);
}
