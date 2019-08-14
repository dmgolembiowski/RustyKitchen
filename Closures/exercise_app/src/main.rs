use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn main() {
    let input: u32 = 8;
    let _intensity: u32 = simulated_expensive_calculation(input);
    println!("Done!");
}
