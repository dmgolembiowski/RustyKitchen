use std::clone;
mod lib;
use lib::NewsArticle;
use lib::Summary;
fn main() {
    struct Generate<T>(fn() -> T);
    impl<T> Copy for Generate<T> {}
    impl<T> Clone for Generate<T> {
        fn clone(&self) -> Self {
            *self
        }
    }
    
    impl Clone for NewsArticle {
        fn clone(&self) -> NewsArticle {
            let n_a = Generate::clone(self);

        }
    }
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championships"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("iceburgh"),
        content: String::from("... the Pittsburgh Penguins once again the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
    let inter_article = clone(article);
    lib::notify(&inter_article);

    lib::bulk_notify(inter_article, inter_article);
}
