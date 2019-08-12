#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait Summary {
    // Commented out line is syntax
    // for enforcing each struct implementation
    // to be uniquely defined, w/o a parent-level 
    // default impl
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
/*
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
*/
impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
