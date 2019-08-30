fn never_returns() -> ! {
    loop {
    }
}

/*
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("Called `Option::unwrap()` on a `None` value"),
        }
    }
}
*/

fn main() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}
