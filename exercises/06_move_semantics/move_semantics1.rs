// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


// move_semantics1.rs

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // To avoid moving vec0 into fill_vec, we can pass a reference to vec0
    // and clone it inside the function to create a new Vec that can be modified.
    let vec1 = fill_vec(&vec0);

    // vec0 should still be valid here, and vec1 should contain the additional element.
    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // Clone the input vector so we have a new instance that we can modify.
    let mut vec = vec.clone();

    vec.push(88);

    vec
}
