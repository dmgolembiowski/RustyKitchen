fn main() {

    let a = [10, 20, 50];

    let mut index = 0; // <- important

    while index < 5 {
        println!("Value: {}", a[index]);

        index += 1;
    }
}
