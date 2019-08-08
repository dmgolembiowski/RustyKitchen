fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut other_v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
