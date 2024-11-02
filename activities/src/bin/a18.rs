// Topic: Result
//
// Requirements:
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message


#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, String> {
        if age >= 21 {
            //   * The Ok variant should contain the initialized structure, but only
            //     if the person is aged 21 or older
            Ok(Self {
                name: String::from(name),
                age: age
            })
        } else {
            //   * The Err variant should contain a String (or &str) that explains why
            //     the structure could not be created
            Err(String::from("New Adult could not be initialized because age is less than 21"))
        }
    }
}

fn create_adult(name: &str, age: u8) -> Result<(), String> {
    let adult = Adult::new(name, age)?;
    println!("{:?}", &adult);
    Ok(())
}

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`

fn main() {
    let child = create_adult("Sanzy", 19);
    let adult = create_adult("Uchechukwu", 24);

    println!("Creating under age adult ...{:?}", child);
    println!("Creating of age adult ...{:?}", adult);
}

