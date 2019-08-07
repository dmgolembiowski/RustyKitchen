enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // has an anonymous struct inside
    Write(String),              // one String
    ChangeColor(i32, i32, i32), // a tuple struct of three values
}

impl Message {
    fn call(&self) {
        // method defined here
        ()
    }

    fn automessage(text: &String) {
        let m = Message::Write(String::from(text));
        m.call();
    }
}

fn main() {
    println!("Hello, world!");
}
