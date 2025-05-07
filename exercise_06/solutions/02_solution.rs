use std::{
    fmt::{self, Display},
    rc::Rc,
};

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
            author,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, By {}", self.title, self.author)
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

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn get_books_by_author(&self, author: &Author) -> Option<Vec<&Book>> {
        let mut books = Vec::new();
        for book in self.books.iter() {
            if std::ptr::eq(&*book.author, author) {
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
    let author_c = Rc::new(Author::new("Author C"));

    let book_1 = Book::new("Book 1", author_a.clone());
    let book_2 = Book::new("Book 2", author_a.clone());
    let book_3 = Book::new("Book 3", author_b.clone());

    library.add_book(book_1);
    library.add_book(book_2);
    library.add_book(book_3);

    let authors = vec![author_a, author_b, author_c];

    for author in authors {
        match library.get_books_by_author(&author) {
            Some(books) => println!("{} books: {:#?}", author, books),
            None => println!("No books found for {}", author),
        }
    }
}
