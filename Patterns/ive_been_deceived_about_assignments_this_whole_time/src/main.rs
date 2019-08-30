fn print_coordinates(&(x, y): &(i32, i32)) -> (i32, i32) {
    (0, 0)
}

fn main() {
    let point = (3,5);
    let point = print_coordinates(&point);
}
