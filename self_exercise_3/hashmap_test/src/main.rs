use std::collections::HashMap;

#[derive(Clone)] // we'll be cloning it later on
struct Node<'a> {
    data: &'a i32 
}


struct Test<'a> {
    hash_map: HashMap<&'a str, Node<'a>>  // the hash map owns the struct
}

impl<'a> Test<'a> {
    fn new() -> Test<'a> {
        Test {hash_map: HashMap::new()}
    }

    fn join(
        &mut self, // must be mutable
        node: Node<'a>) { // do not pass a reference
        self.hash_map.insert("test", node);  // inserting moves `node`
    }
}

fn main() {
    let stuff = Node {data: &12};
    let mut test = Test::new();

    test.join(stuff.clone());  // if we don't clone, `stuff` will get moved
    println!("{}", test.hash_map["test"].data); 
    //println!("{}", *test.hash_map["test"].data);  // outputs "12"
}
