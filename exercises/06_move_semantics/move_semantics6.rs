// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Pass a reference to data instead of moving it

    string_uppercase(data); // Now data is moved here
}

// Should not take ownership, so we change the parameter to a reference to String
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership, so the parameter should be a String, not a reference
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // We own data, so we can directly assign the uppercase version to it

    println!("{}", data);
}


//WRITEUP: 
//The function get_char was taking ownership of the data when it should only borrow it
//So i changed the signature of get_char to borrow data by changing its parameter to a reference (&String)