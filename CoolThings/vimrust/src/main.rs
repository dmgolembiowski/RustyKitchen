use std::process::Command;

fn main() {
    Command::new("vim").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
}
