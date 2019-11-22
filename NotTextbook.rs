/* 
// "Textbook" Rust: 

fn main() {
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(&m == &(&mut 64));
    println!("Look, I can still use m! m = {:?}", &m);
    *m = *m + 192; // Seems that this really is equivalent to `+=` operation
    println!("Look, I did it again! m = {:?}", &m);
}

// It looks so easy, elegant, and instructive. And examples like these are, but 
// it's not entirely clear how you're supposed to take this kind of
// example and just suddenly know how to pass variables to other functions,
// using valid signatures in those external function calls, and do normal
// programming logic without screwing everything up.

// For me, building type-safe logic in Rust is *really* hard (and I've read the
// book from cover to cover)!
// What's more, building function signatures in Rust (that let you re-use variables)
// is even harder. I always manage to get some error that's telling me to add a
// missing lifetime specifier, or that I gave a `&(&mut <Thing>)` when my function
// provided a `&mut <Thing>`.

// As an inexperienced Rust programmer, I want to just operate on 
// my data like I would in Python, Javascript, C#, BASH, etc. 
// But here's the thing, writing Rust is fundamentally harder because
// the coding patterns are different, so this demonstration hopes to show
// a pattern that works: one that actually re-uses variable names and calls
// functions outside of main to mutate data.
*/


// Here we're starting with the textbook example:
// ----------------------------------------------
fn main() {
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(&m == &(&mut 64));
    println!("Look, I can still use m! m = {:?}", &m);
    *m = *m + 192; // Seems that this really is equivalent to `+=` operation
    println!("Look, I did it again! m = {:?}", &m);
}

// Except now, I'm replacing every assignment with an external function call:
// --------------------------------------------------------------------------
fn shared_mut_ref_v2(y: &mut i32) -> &mut i32 {
    // A scope that simulates actual things being
    // done to y
    y
}

fn first_print(m: &(&mut i32)) {
    // A scope that doesn't return anything, but
    // executes a Shell job that also potentially
    // needs to mutate `m` to be something else
    println!("m = {:?}", m);
}

fn add_one_hundred_and_ninety_two(m: &mut i32) -> i32 {
    // A scope that does a huge algorithim to update
    // a variable's value
    *m + 192
}

fn assert_equals_256(ref_m: &(&mut i32)) -> bool {
    // Recreating `assert!(&m == &(&mut 64));`
    match ref_m == &(&mut 256) {
        true => true,
        false => false,
    }
}

fn main() {
    /*
        You probably want to write functions in your
        programs, and not just write short, zingy
        one-liners that do magical textbook-magic.
        
        
    */
    let mut y = 32 as i32;
    //      y: mut i32

    let m = shared_mut_ref_v2(&mut y);
    //  m : &mut i32

    *m += add_one_hundred_and_ninety_two(m);
    /* 
        The above statement is (almost) identically equal to:
            *m = *m + add_one_hundred_and_ninety_two(m);
    */

    first_print(&m);
    // This call doesn't eat up `m`; the consequence is that our
    // function's argument's type annotation looks grosser

    if assert_equals_256(&m) {
        println!("Look, I can still use m! m = {:?}", &m);
    } else {
        println!("Guess not; m = {:?}", &m);
    }
    
    /* I find `match` statements hard to use sometimes;
       so here's a demo using them instead of the `assert` macro */
    let mut def_not_256 = 6567 as i32;
    //      def_not_256: mut i32
    
    let rly_def_not_256 = &mut def_not_256;
    //  rly_def_not_256: &mut i32

    // Test things out:
    if assert_equals_256(&rly_def_not_256) {
        println!("Look! 256 = {:?}", &rly_def_not_256);
    } else {
        println!("See, 256 is clearly not equal to {:?}", &rly_def_not_256);
    }
}
