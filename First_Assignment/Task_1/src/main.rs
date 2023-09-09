/*
Define the Person struct
*/
#[allow(dead_code)]
struct Person {
    age: usize,
    name: String,
}

// Define the Book struct
#[allow(dead_code)]
struct Book {
    title: String,
    author: String,
    is_available: bool,
}

// Define the Library struct that holds a list of books
#[allow(dead_code)]
struct Library {
    books: Vec<Book>,
}

fn main() {
    println!("Hello, world!");
}