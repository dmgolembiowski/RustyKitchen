use std::collections::HashMap;

#[allow(dead_code)]
struct Cache<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
    internal: HashMap<u32, u32>
}

#[allow(dead_code)]
impl<T> Cache<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
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
        /*if self.internal.contains_key(&arg) {
            Some(self.get(&arg))
        } else {
            let v = self.calculation(&arg);
            self.set(arg, &(self.calculation)(arg));
            Some(self.internal[*arg])
        }*/
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

fn main() {
    println!("Hello, world!");
}

