// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // Clone vec0 before passing it to fill_vec to avoid moving vec0.
    let vec1 = fill_vec(vec0.clone()); // Ensure this line is saved in the file.

    // Now we can use vec0 after fill_vec because it was cloned, not moved.
    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

//WRITEUP : 
//vec0 is moved into fill_vec function so its no longer usable after the function call, so i just sopied vec0 right before passing it to fill_vec so we keep control of the original vec0 var
