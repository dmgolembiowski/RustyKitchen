#[allow(dead_code)]
struct Book {
    title: String,
    author: String,
    rating: i32,
    online: bool,
}

#[allow(dead_code)]
fn build_book(title: String, author: String, rating: i32) -> Book {
    Book {
        title,
        author,
        rating,
        online: true,
    }
}

#[allow(dead_code)]
fn main() {
    let book1 = build_book(String::from("Water"), String::from("Avatar"), 1);
    let bookI = Book {
        title: String::from("Air"),
        author: book1.author,
        rating: 1,
        online: true,
    };
    bookI.title.as_bytes_mut
}
