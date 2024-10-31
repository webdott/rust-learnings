// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:

fn main() {
    // * Use a mutable integer variable
    let mut i = 1;
    
    // * Use a loop statement
    loop {
        if i > 4 {
            // * Use break to exit the loop
            break;
        }
        
        // * Print the variable within the loop statement
        println!("{:?}", i);
        i = i + 1
    }

}

