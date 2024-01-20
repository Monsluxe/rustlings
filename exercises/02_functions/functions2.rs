// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

//On ajoute le paramètre num à la fonction call_me, avec le type de donnée qui va avec (i32 pour une int en 32bits)
//Le print est deja correctement géré, donc une fois la signature de call_me complete, l'output est correct, le code compile