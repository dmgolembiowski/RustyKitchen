use std::io;
/* Like C#, could have done 
 * `use std::io::stdin;` at
 * the top to just call
 * `stdin()` at the bottom.
 */
fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let mut guess = String::new(); //`String` -> "growable" & UTF-8 encoded text

    // Call std::io::Stdin().read_line()

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    /* `io::stdin()` returns an instance of `std::io::Stdin`,
     * a type representing a handle to the STDIN for bash. */

    /* The mysterious `&` used in `&mut guess`
     * assigns a reference to a mutable copy
     * of the immutable variable `guess`*/

    println!("You guessed: {}", guess);
}
