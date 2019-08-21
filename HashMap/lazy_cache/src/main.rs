use std::collections::HashMap;

pub struct Cache<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    internal: HashMap<u32, u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cache<T> {
        Self {
            calculation,
            internal: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        let Self { internal, calculation } = self;
        let entry = internal.entry(arg);
        *entry.or_insert_with(|| (calculation)(arg))
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
    assert_eq!(v2, 2);
}
