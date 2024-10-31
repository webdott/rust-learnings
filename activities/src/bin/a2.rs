// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn sum_two_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
// * Use a function to display the result
fn print_sum(result: i32) {
    println!("{:?}", result)
}
// * Use the "{:?}" token in the println macro to display the result
fn main() {
    print_sum(sum_two_numbers(3, 2));
    println!("The sum of {:?} and {:?} is {:?}", 3, 2, sum_two_numbers(3, 2))
}




