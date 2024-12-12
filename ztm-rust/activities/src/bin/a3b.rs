// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively

// Notes:

fn main() {
    // * Use a variable set to any integer value
    let set_variable = 2;
    
    // * Use an if..else if..else block to determine which message to display
    if set_variable > 5 {
        println!(">5");
    } else if set_variable == 5 {
        println!("=5");
    } else {
        // * Use the println macro to display messages to the terminal
        println!("<5");
    }
}

