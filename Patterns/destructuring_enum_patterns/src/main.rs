enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0,160, 255);

    match msg {
        Message::Quit => { },
        Message::Move{x, y} => {},
        Message::Write(text) => { println!("{}", text);},
        Message::ChangeColor(r, g, b) => { println!("{} {} {}", r, b, b); }
    }
}
