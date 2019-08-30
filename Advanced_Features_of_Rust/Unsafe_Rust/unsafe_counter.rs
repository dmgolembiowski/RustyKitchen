static mut COUNTER: u32 = 1;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
        }
}

fn main() {
    add_to_count(4_000);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

