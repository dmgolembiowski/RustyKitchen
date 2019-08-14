extern crate rand;

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation() -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    rand::random::<u32>()
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("{}", simulated_expensive_calculation(intensity));
        println!("{}", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("{}", simulated_expensive_calculation(intensity));
        }
    }

}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = rand::random::<u32>();
    let _intensity: u32 = simulated_expensive_calculation();
    println!("Done!");
}
