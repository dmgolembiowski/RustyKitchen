
/*  Write a function that takes a string and returns the first word it finds 
 *  in that string. If the function doesn’t and a space in the string, 
 *  the whole string must be one word, so the entire string should be returned.
 */


// Tedious if s.clear() gets called, thus drop()-ing
// the value completely
fn first_word(s: &String) -> usize {
    // (With what I know now) Since the 
    // Rust language's strictness prevents me 
    // from returning part of a string, this will
    // instead return an index!
    let bytes = s.as_bytes();

    // if (s ~= "string with several words")
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    /* `bytes.iter().enumerate()`
     * == (b'f', b'i', b'r', b's', b't', b' ', b'w', b'o', b'r', b'd')
     */

    // elif (s ~= "oligoliteral")
    s.len()
}
// Gets even worse trying to use:
//      fn second_word(s: &String) -> (usize, usize) {}
// to track a starting and ending index! The solution...

fn best_first(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
/* Good grief! What a time eater!
fn final_first(s: &str) -> &str {
    // let bytes = s.as_bytes(); 
    // Do I make this instead as `s.as_bytes_mut()`?
    for (i, &item) in s.enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
}
*/
fn main() {
    let name = String::from("David Golembiowski");
    let digit = String::from("David Golembiowski");
    println!("The first word of 'David Golembiowski' is {}", best_first(&name));
    println!("The break occurs at the {} character", first_word(&digit));
}
