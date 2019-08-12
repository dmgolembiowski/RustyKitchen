mod lib;
use lib::NewsArticle;
use lib::Summary;

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championships"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("iceburgh"),
        content: String::from("... the Pittsburgh Penguins once again the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    lib::notify(article);
}
