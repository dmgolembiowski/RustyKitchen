
/// A simplified `vec!` macro from the stdlib
#[macro_export] // bring into scope
macro_rules! vec {
    // First a set of parenthesees (...) encompasses the whole pattern to be matched
    //
    // Next, Within `$()` is `$x:expr`, which matches any Rust expression and gives
    // the expression the name `$x`
    //
    // I'm guessing that `,*` implies that all other $(), $(), ... patterns coming 
    // after `$( $x:expr )` are globbed, which also includes the empty pattern.
    //
    // My guess was close: "The `*` specifies that the pattern matches zero or more 
    // of whatever precedes the `*`.
    ( $( $x:expr  ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);        
            )*
            temp_vec
            /* // A call like `let g = vec![1, 2, 3];` yields:
             * let mut temp_vec = Vec::new();
             * temp_vec.push(1);
             * temp_vec.push(2);
             * temp_vec.push(3);
             * temp_vec
             */
        }
    };
}

fn main() {
    println!("Hello, world!");
}
