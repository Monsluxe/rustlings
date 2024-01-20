// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // Fields for the struct
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8); // Tuple struct with three u8 elements

#[derive(Debug)]
struct UnitLikeStruct; // Unit-like struct

#[cfg(test)]
mod tests {
    // Bring the structs into scope for the tests
    use super::{ColorClassicStruct, ColorTupleStruct, UnitLikeStruct};

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic c struct with the specified color values for green
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct with the specified color values for green
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}

