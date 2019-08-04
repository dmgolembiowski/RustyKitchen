extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1, 4);

    let mut guess = String::new();
    let checker = |s_n: u32| -> bool {
            match s_n.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Low");
                    return false;
                },
                Ordering::Greater => {
                    println!("High");
                    return false;
                },
                Ordering::Equal => {
                    println!("You win!");
                    return true;
                }
            }
        };
    loop {
        println!("Please type a number: ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        /*
        let checker = |s_n: u32| {
            match s_n.cmp(&secret_number) {
                Ordering::Less => println!("Low"),
                Ordering::Greater => println!("High"),
                Ordering::Equal => {
                    println!("You win!");
                },
            }
        }; */
        match guess
            .trim()
            .parse() {
                Ok(valid_u32_thing) => {
                    println!("You entered {}", guess);
                    if checker(valid_u32_thing) {
                        break;
                    } else { continue; } // This does not work, need a `pass` statement
                },
                Err(_) => {
                    println!("You entered {}", guess);
                    continue;
                },
            };
    }
}
