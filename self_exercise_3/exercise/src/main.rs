use std::thread;
use std::time::Duration;


struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!",
                 expensive_closure(intensity));
        println!("Next, do {} situps!", 
                 expensive_closure(intensity));
    } else {
        if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", 
                     expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_value = 10;
    let simulated_rand_no = 5;

    generate_workout(
        simulated_user_value,
        simulated_rand_no
    );
}
