// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal

// Notes:

// * Use an enum with color names as variants
enum Color {
    Red,
    Blue,
    Green,
}

// * Use a function to print the color name
fn print_color(color: Color) {
    // * The function must use the enum as a parameter
    
    // * Use a match expression to determine which color
    match color {
        //   name to print
        Color::Blue => println!("Blue!"),
        Color::Green => println!("Green!"),
        Color::Red => println!("Red!"),
    }
}

fn main() {
    print_color(Color::Blue);
    print_color(Color::Red);
    print_color(Color::Green);
}

