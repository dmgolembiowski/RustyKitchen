// Like Python, functions can be declared in any order
fn main() {
    fizzbuzz_to(100);
}

fn divisible_by(leftHS: u32, rightHS: u32) -> bool {
    // Core case, early return
    if rightHS == 0 {
        return false;
    }

    // B/c it's an expression, 
    // the `return` keyword isn't needed
    leftHS % rightHS == 0
}

// Functions that "don't" return a
// value actually return the unity type '()'
fn fizzbuzz(n: u32) -> () {
    if divisible_by(n, 15) {
        println!("FIZZBUZZ");
    } else if divisible_by(n, 3) {
        println!("fizz");
    } else if divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function return `()`, the return
// type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    // for(int n = 1; n < 100+1; n++)
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

