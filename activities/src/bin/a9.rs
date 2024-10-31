// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5

// Notes:

// * Use a function that returns a tuple
fn coordinate(x: i32, y:i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    // * Destructure the return value into two variables
    let (_, y) = coordinate(5, 5);
    
    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("Greater than 5")
    } else if y < 5 {
        println!("Less than 5")
    } else {
        println!("Equal to 5")
    }
}

