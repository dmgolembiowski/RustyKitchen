enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0,160, 255));

    match msg {
       Message::ChangeColor(Color::Rgb(r, g, b)) => { println!("{} {} {}", r, b, b) },
       Message::ChangeColor(Color::Hsv(h, s, v)) => {
           println!(
               "Change the color to huse {}, saturation {}, and value {}",
               h,
               s,
               v
               )
       }
       _ => ()
    }
}
