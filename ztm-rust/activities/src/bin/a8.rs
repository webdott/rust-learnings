// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Strawberry,
    Apple,
    Orange,
    Mango
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Apple => println!("Apple"),        
        Flavor::Orange => println!("Orange"),        
        Flavor::Strawberry => println!("Strawberry"),        
        Flavor::Mango => println!("Mango"),        
    };

    println!("{:?}", drink.ounce);
}

fn main() {
    let strawberry_srink = Drink {
        flavor: Flavor::Strawberry,
        ounce: 3.10
    };

    print_drink(strawberry_srink);
}

