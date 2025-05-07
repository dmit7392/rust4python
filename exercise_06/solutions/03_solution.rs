use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

/* do not modify this struct */
#[derive(Debug)]
struct Author {
    name: String,
    email: Option<String>,
}

/* do not modify this impl block */
impl Author {
    fn new(name: &str) -> Self {
        Author {
            name: name.to_string(),
            email: None,
        }
    }

    fn set_email(&mut self, email: &str) {
        self.email = Some(email.to_string());
    }
}

/* do not modify this impl block */
impl Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(email) = &self.email {
            write!(f, "{} [{}]", self.name, email)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/* you may modify this struct */
#[derive(Debug)]
struct Book {
    title: String,
    author: Rc<RefCell<Author>>,
}

/* you may modify this impl block */
impl Book {
    // do not modify this method
    fn new(title: &str, author: Rc<RefCell<Author>>) -> Self {
        Book {
            title: title.to_string(),
            author,
        }
    }
}

/* you may modify this impl block */
impl Display for Book {
    // do not modify this method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, By {}", self.title, self.author.borrow())
    }
}

/* do not modify this struct */
#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    // do not modify this method
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    // do not modify this method
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // you may modify this method
    fn get_books_by_author(&self, author: &Author) -> Option<Vec<&Book>> {
        let mut books = Vec::new();
        for book in self.books.iter() {
            let book_author = book.author.borrow();
            // `std::ptr::eq` returns true if the two pointers point to
            // the same memory location
            if std::ptr::eq(&*book_author, author) {
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

    // the author variables must remain immutable
    let author_a = Rc::new(RefCell::new(Author::new("Author A")));
    let author_b = Rc::new(RefCell::new(Author::new("Author B")));
    let author_c = Rc::new(RefCell::new(Author::new("Author C")));

    // the book variables must remain immutable
    let book_1 = Book::new("Book 1", author_a.clone());
    let book_2 = Book::new("Book 2", author_a.clone());
    let book_3 = Book::new("Book 3", author_b.clone());

    library.add_book(book_1);
    library.add_book(book_2);
    library.add_book(book_3);

    let authors = vec![&author_a, &author_b, &author_c];

    for author in &authors {
        let author = author.borrow();
        match library.get_books_by_author(&author) {
            Some(books) => println!("{} books: {:#?}", author, books),
            None => println!("No books found for {}", author),
        }
    }

    // - add code here to update the email addresses of the authors
    // - do not modify code blocks where it says "do not modify", all
    //   other code can be modified
    // - assign the email address "author_a@mybooks.local" to author A
    // - assign the email address "author_b@mybooks.local" to author B
    {
        // let mut author_a_mut = (*author_a).borrow_mut();
        author_a.borrow_mut().set_email("author_a@mybooks.local");

        let mut author_b_mut = author_b.borrow_mut();
        author_b_mut.set_email("author_b@mybooks.local");
    }

    for author in &authors {
        let author = author.borrow();
        match library.get_books_by_author(&author) {
            Some(books) => println!("{} books: {:#?}", author, books),
            None => println!("No books found for {}", author),
        }
    }
}
