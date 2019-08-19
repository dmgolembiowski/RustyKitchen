use std::collections::HashMap;

#[allow(dead_code)]
struct Cache<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    internal: HashMap<u32, u32>
}

#[allow(dead_code)]
impl<T> Cache<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            internal: HashMap::new(),
        }
    }

    fn set(&mut self, arg: u32, value: u32) -> u32 {
        self.internal.insert(arg, value);
        self.get(arg)
    }

    fn get(&mut self, arg: u32) -> u32 {
        self.internal[&arg]
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.internal.contains_key(&arg) {
            true => {
                self.get(arg)
            },
            false => {
                //let calculation = self.calculation;
                self.set(arg, (self.calculation)(arg))
            },
        }
    }
}

#[allow(dead_code)]
#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

fn main() {
    println!("Hello, world!");
}

