use std::sync::Mutex;

fn main() {
    let m = Mutex::new(50);

    {
        let mut num = m.lock().unwrap();
        //          = LockResult<MutexGuard<T>>.unwrap();
        //          = MutexGuard<T>; //such that there exists
        //          num::Deref (...) -> num::......????
        //
        //          To figure out what was happening at the atomic level,
        //          I looked over this Stack Overflow question. At the present moment,
        //          I think this code could could be reduced to 
        //
        //             &mut *m.lock().unwrap() += 1;
        //
        //          thus performing an implicit borrow, which... to me sounds like
        //          a situation where a variable is temporarily referenced, 
        //          then used in a statement, then dereferenced. It almost begs the 
        //          question, does performing an implicit borrow on a Mutex unlock the data as it's
        //          being written?
        *num += 1;
    }

    println!("m = {:?}", m);
}
