use std::fmt::Display;

pub trait Yalpsid {
    fn pass() {
        ()
    }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display 
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x    
    } else {
        y
    }
}

fn main() {
    let x = "three";
    let y = "five";
    let longest = longest_with_an_announcement(x,y);
    println!("{} {} -> {}", "three", "five", longest_with_an_announcement("three", "five");
}
