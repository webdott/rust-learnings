// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
fn display_first_name() -> &'static str {
    return "Uchechukwu";
}
// * Use a function to display your last name
fn display_last_name() -> &'static str {
    return "Nwafor";
}
// * Use the println macro to display messages to the terminal
fn main() {
    println!("My name is {:?} {:?}", display_first_name(), display_last_name());
}





