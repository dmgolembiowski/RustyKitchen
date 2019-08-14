
// All iterators implement a trait named
// `Iterator` in the standard lib, w/
// a definition like:
pub trait Iterrator {
    type Item; /* <- Implementing an interator
                  requires that the programmer
                  also defines an `Item` type
                  to be used in the return type
                  of the `next()` method */

    // Next must return successive values wrapped
    // in a Some(...) and when iteration is over,
    // return `None`
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    /* Note
     * ----
     *
     * */
    assert_eq!(v2, vec![2, 3, 4]);
}
