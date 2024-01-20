// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}


//WRITEUP 
//In the script, basically the y var was moved into match function, making it inacessible after that, i just created another variable that stay more global, and store the var data of y variable 