// Define a struct called Book
struct Book {
    pages: i32,
    ratings: i32,
}

// Function to display the page count of a book
fn display_page_count(book: &Book) {
    // Borrow a reference to the book's pages
    println!("The book has {} pages", book.pages);
}

// Function to display the ratings of a book
fn display_ratings(book: &Book) {
    // Borrow a reference to the book's ratings
    println!("The book has {} ratings", book.ratings);
}

fn main() {
    // Create a new book instance
    let book = Book {
        pages: 100,
        ratings: 100,
    }; // `book` is the owner of the Book struct

    // Borrow a reference to the book and pass it to the display_page_count function
    display_page_count(&book);

    // Borrow a reference to the book and pass it to the display_ratings function
    display_ratings(&book);
}
