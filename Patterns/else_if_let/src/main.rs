fn main() {
    let fav_colo: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_colo { println!("{}", color); }

    else if is_tuesday { println!("It's Tuesday"); }

    else if let Ok(age) = age { if age > 30 { } else { } }

    else { }
}
