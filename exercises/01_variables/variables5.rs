// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = number.chars().rev().collect::<String>();

    match number.parse::<i32>() {
        Ok(i) => println!("Number plus two is : {}", i + 2),
        Err(e) => println!("Error parsing number: {}", e),
    }
}
//WRITEUP
//Here we using the shadowing in order to redeclare a variable with anoter value
//We convert the string of the number to a String type and reverse the characters using the chars().rev().collect::<String>() method