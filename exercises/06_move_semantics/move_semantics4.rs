#[test]
fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.push(88);

    vec
}
