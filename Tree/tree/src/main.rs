use std::rc::Rc;
use std::cell::RefCell;

/// Since we want a node to own its children,
/// and want to share ownership so that each kid 
/// can be accessed directly
#[derive(Debug)]
struct Node {
    /* Define the Vec<T> items to be values
     * of the type Rc<Node>; also want to indicate
     * which nodes are parents of children,
     * so it uses RefCell<T> wrapping vec![Rc::clone(&child)] */
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 4,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
