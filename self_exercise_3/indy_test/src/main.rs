use std::collections::HashMap;

#[allow(dead_code)]
struct Cache<'a, T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    internal: HashMap<&'a u32, &'a u32>
}

impl<'a, T> Cacher<'a, T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<'a, T> {
        
    }
}

fn main() {
    println!("Hello, world!");
}
