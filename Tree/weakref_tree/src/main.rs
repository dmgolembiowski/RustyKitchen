use std::rc::{Rc, Weak};
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
    /* Need particular type for parent such that when
    *  a parent node is dropped, so are each of its children,
    *  but not the other way around.
    *  ((Therefore, going to use a weak reference!
    *  Also, a parent node should own data in its children. */
    parent: RefCell<Weak<Node>>,    
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}",
             leaf.parent.borrow()
             .upgrade());
    // if .upgrade()->None then parent is None
    let branch = Rc::new(Node {
        value: 4,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // Use Rc::downgrade fn to make a Weak<Node> ref to
    // branch from the Rc<Node> in branch
   *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 

    println!("leaf parent = {:?}",
             leaf.parent.borrow()
             .upgrade());
}
