// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under

// Notes:

// * Use a struct for a persons age, name, and favorite color
struct Person {
    // * The color and name should be stored as a String
    name: String,
    fav_color: String,
    age: i32
}

fn print_name_colors(person: &Person) {
    // * The name and colors should be printed using a function
    println!("Name: {:?}; Favorite color: {:?}", person.name, person.fav_color)
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("Uchechukwu"),
            fav_color: String::from("Purple"),
            age: 24
        },
        Person {
            name: String::from("Azeez"),
            fav_color: String::from("Blue"),
            age: 23
        },
        Person {
            name: String::from("Nonse"),
            fav_color: String::from("Black"),
            age: 24
        },
    ];
        
    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age >= 24 {
            print_name_colors(&person);
        }
    }
}
    
