


// Define a lifetime 'a and assign it to both fields in the struct.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // The Book instance and its fields share the same lifetime.
    let book = Book {
        author: &name, // `&name` has the same lifetime as `name`
        title: &title, // `&title` has the same lifetime as `title`
    };

    println!("{} by {}", book.title, book.author);
}

//Writeup

//The Book struct was defined without lifetimes  =compil error
//So i added a lifetime param "a" in the book struct, now author and titles will have the same lifetime referenced. 