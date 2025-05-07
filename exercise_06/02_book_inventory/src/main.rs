// Note: For this exercise, be sure to review `07_structs` in module 3

use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Debug)]
struct Author {
    name: String,
}

impl Author {
    fn new(name: &str) -> Self {
        Author {
            name: name.to_string(),
        }
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    author: Rc<Author>,
}

impl Book {
    fn new(title: &str, author: Rc<Author>) -> Self {
        Book {
            title: title.to_string(),
            author//: Author::new(author),
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, author: Rc<Author>, book_title: &str) {
        let book = Book::new(book_title, author);
        self.books.push(book);
    }

    fn get_books_by_author(&self, author: &Author) -> Option<Vec<&Book>> {
        let mut books = Vec::new();
        for book in self.books.iter() {
            if book.author.name == author.name {
                books.push(book);
            }
        }
        if !books.is_empty() {
            Some(books)
        } else {
            None
        }
    }

}

fn main() {
    let mut library = Library::new();

    let author_a = Rc::new(Author::new("Author A"));
    let author_b = Rc::new(Author::new("Author B"));

    library.add_book(author_a.clone(), "Book 1");
    library.add_book(author_b.clone(), "Book 2");
    library.add_book(author_b.clone(), "Book 3");

    let authors = vec![author_a, author_b,
                                        Rc::new(Author::new("Author C"))];

    for author in authors {
        match library.get_books_by_author(&author) {
            Some(books) => println!("{} books: {:#?}", author, books),
            None => println!("No books found for {}", author),
        }
    }
}

// Review the code above. How many "Author A" struct instances are created?
// Update this code to create each author struct only once, and re-use the
// struct across multiple books. To search for books written by a particular
// author, update the `library.get_books_by_author` method to receive an
// `Author` struct instead of an author name. The application is
// single-threaded.
