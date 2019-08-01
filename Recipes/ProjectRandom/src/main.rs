extern crate rand;

use rand::{thread_rng, Rng};
use rand::distribution::Alphanumeric;

fn main() {
    let rand_string: Strign = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    println!("{}", rand_string);
}
