# RustyKitchen

FINISHED reading THE ENTIRE BOOK (page 593/593)
<br></br>
<br></br>

To do:
+ Per page 566, try to write a version of `new` for the `ThreadPool` with the signature `pub fn new(size:usize) -> Result<ThreadPool, PoolCreationError> { ....`
+ Continue working through /Blog/blog/src/lib.rs
+ Create a program that intentionally creates a memory cycle!

+ ++ Find examples of situations where the three rules of deref coercion are demonstrated (maybe create some)
+ Notate *interior mutability* patterns
+ Notate *reference cycles*

Interesting Finds:
------------------
"if let" versus "match": They are basically the same thing, except 
that it seems "match" uses a low-level "discriminant" function. 
For more, read: http://patshaughnessy.net/2018/1/18/learning-rust-if-let-vs--match
<br />
Smart Pointer Types (SMPT)
-----------------------------
1) Reference Counting SMPTs: Enables data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.
    These actually include `String` and `Vec<T>`. *Note: the characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers 
    implement the* `Deref` *and* `Drop` *traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference! By contrast, the
    Drop trait allows the programmer to flexibly configure how the smart pointer goes out of scope once the code is already running.*

