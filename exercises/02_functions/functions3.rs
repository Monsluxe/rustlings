// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
//WRITEUP
//Here, the call_me function is well initialized, and made to take an u32 argument, but when its called, its done without any argument, so lets try to add an argument to the call in the main function.