mod library {
    mod model {
        pub struct Book {
            pub title: String,
        }
    }

    mod search {
        use super::model::Book;

        pub fn title(book: &Book) -> &str {
            &book.title
        }
    }

    // ANCHOR: reexport
    pub use model::Book;
    pub use search::title;
    // ANCHOR_END: reexport
}

fn main() {
    let book = library::Book {
        title: String::from("Rust"),
    };
    assert_eq!(library::title(&book), "Rust");
}
