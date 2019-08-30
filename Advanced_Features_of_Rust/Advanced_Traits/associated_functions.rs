trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called ~not~ a {}", Dog::baby_name());
    println!("FALSE!");
    println!("actually just {}", <Dog as Animal>::baby_name());
    // In general, fully qualified syntax is defined as follows:
    /// <Type as Trait>::function(receiver_if_method, next_arg, ...);

}
