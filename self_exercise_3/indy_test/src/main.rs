use std::collections::HashMap;

#[allow(dead_code)]
struct Cache<'a, T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    internal: HashMap<&'a u32, &'a u32>
}

#[allow(dead_code)]
impl<'a, T> Cache<'a, T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cache<'a, T> {
        Cache {
            calculation,
            internal: HashMap::new(),
        }
    }

    fn set(&mut self, arg: &'a u32, value: &'a u32) {
        self.internal.insert(arg, value);
    }

    fn get(&mut self, arg: &'a u32) -> &'a u32 {
        self.internal[arg]
    }
}

fn main() {
    println!("Hello, world!");
}
