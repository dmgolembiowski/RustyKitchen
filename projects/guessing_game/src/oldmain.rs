extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

/* Like C#, could have done 
 * `use std::io::stdin;` at
 * the top to just call
 * `stdin()` at the bottom.
 */

fn main() {
   let secret_number = rand::thread_rng()
        .gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let mut guess = String::new(); //`String` -> "growable" & UTF-8 encoded text
    let mut guess: u32 = guess.trim().parse()
        .expect("Please type a number!")

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // this syntax is like the `assert X, "print this"`

    /* `io::stdin()` returns an instance of `std::io::Stdin`,
     * a type representing a handle to the STDIN for bash. */

    /* `::` syntax means "associated function", which is
     * comparable to a "static method" in other languages. */

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You guessed correctly!"),
    }

}
