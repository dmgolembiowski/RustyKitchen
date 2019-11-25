use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct Thing<A>(A);
impl<A> Thing<A> {
    fn new(a: A) -> Thing<A> {
        Thing(a)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Stuff<B>(B);
impl<B> Stuff<B> {
    fn new(b: B) -> Stuff<B> {
        Stuff(b)
    }
}

fn main() {

    let a = Rc::new(Thing::new(12));
    let b = Stuff::new(Rc::clone(&a));
    let c = Rc::clone(&Rc::new(Stuff(String::from("a"))));
    //let nest = |var| {
    //    Rc::clone(&Rc::new(var))
    //};

   
    println!("a={:?} b={:?} c={:?}", *a, *b.0, *c);
}


