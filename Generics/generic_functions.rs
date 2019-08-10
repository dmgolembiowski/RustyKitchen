/*
 * The function, largest, is generic over
 * some type T. This function has one
 * parameter named `list`, which is a slice
 * of values of type T. The largest function
 * will return a value of the same type T. */
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// Finally, this can be more convenient
fn main() {
    let nums = vec![1,2,3,4,432,4];
    let result = largest(&nums);
    println!("The largest number is {}", result);
    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("{}", result);
}
