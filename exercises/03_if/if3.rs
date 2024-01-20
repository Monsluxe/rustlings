// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2 // Changed from 2.0 to 2 to ensure the same type is used
    } else if animal == "snake" {
        3
    } else {
        4 // Changed from "Unknown" to 4 to ensure the same type is used
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}


//WRITEUP:
// animal_habitat function takes an animal name (string) and return his habitat. the 'identifier' var use different type of value, inside different scope of the if statement, rust dont allow this, so lets change the logic processing 