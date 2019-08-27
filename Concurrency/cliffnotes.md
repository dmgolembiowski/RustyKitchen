# Fearless Concurrency:
-----------------------

# What does `std::thread::spawn(X)` do?
+ Accepts a closure (lambda) in place of X, where X is the behavior to be executed.

# How do I prevent the main thread from racing ahead of extraneously spawned threads?
+ Call `spawned_thread_name.join().unwrap()` before the end of their scope... or maybe... before their lifetime expires...? (*wonders mischeviously*)

# How do I write serial code that can have portions run concurrently?
+ *Answer: Start with the more general question* 
+    -> *How do I make the main thread wait for spawned threads to finish before running the rest of its code:*

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi num {} from the spawned thd", i);
            thread.sleep(Duration::from_millis(1));    
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

