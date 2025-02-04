// Mutable data can be mutably borrowed using &mut T.
// This is called a mutable reference and gives read/write access to the borrower

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutable_book`
    let immutable_book = Book {
        // String literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutable_book` and call it `mutable_book`
    let mut mutable_book = immutable_book;

    // Immutably borrow an immutable object
    borrow_book(&immutable_book);

    // Immutably borrow a mutable object
    borrow_book(&mutable_book);

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_book);

    // Can't borrow an immutable object as mutable
    // new_edition(&mut immutable_book);
}
