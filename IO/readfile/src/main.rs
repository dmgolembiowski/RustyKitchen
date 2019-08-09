use std::fs;

fn open_file(filename: &str) -> String {
    // let fileName = String::from(&filename);
    let err_message = format!("Failed to open {}", filename);
    let content = fs::read_to_string(filename)
        .expect(&err_message);
    return String::from(content)
}

fn main() {
    let _filename = String::from("example.txt");
    println!("The file contained: {}", open_file(&_filename));
}
