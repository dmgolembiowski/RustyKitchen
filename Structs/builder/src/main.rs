#[allow(dead_code)]
struct Book {
    title: String,
    author: String,
    rating: i32,
    online: bool,
}

#[allow(dead_code)]
fn verbose_build_book(title: String, author: String, rating: i32) -> Book {
    Book {
        title: title,
        author: author,
        rating: rating,
        online: true,
    }
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

fn main() {
    println!("Read the file!!");
}
