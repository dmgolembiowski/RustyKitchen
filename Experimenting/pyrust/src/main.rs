#![feature(proc_macro_hygiene)]
use inline_python::python;
pub mod context;

fn main() {
    println!("Hello, world!");
    let who = "world";
    let n = 5;
    python! {
        for i in range('n):
            print(i, "Hello", 'who)
        print("Goodbye")
    }

    // Calling context.rs
    context::test();
}
