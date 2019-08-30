let mut num = 5;
/* Below, using `as` to cast an immutable and mutable 
 * reference into their corresponding raw pointer types.
 *
 * B/C I created them directly from references
 * guaranteed to be valid, we know these particular 
 * raw pointers are valid, but we can't make that assumption
 * about any regular raw pointer */
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

/* Here's how you might create a raw pointer to an arbitrary 
 * location in memory. Trying to use arbitrary memory 
 * is undefined: 
 *      * This might cause a seg-fault because there might
 *      be memory at that address or there might not!
 *      It's generally a bad idea unless you have some 
 *      guarantees that it'll work.
 */
let address = 0x012345usize;
let r = address as *const i32;
