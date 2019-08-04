fn main() {
     /* (Modify this for a more comprehensive explanation later) */
 
     /* The double colon operator lets you namespace(verb)
      * this `from()` function under the `String` type
      * instead of using some new name like "string_from()"
      */
     let mut s = String::from("hello");
 
     // Mutating `s`
     s.push_str(", world!"); // push_str() appends a literal to a String

     prinln!("{}", s); // Out: hello, world!
}
