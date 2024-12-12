// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;


fn main() {
    let mut hashmap = HashMap::new();

    hashmap.insert("Chairs", 5);
    hashmap.insert("Beds", 3);
    hashmap.insert("Tables", 2);
    hashmap.insert("Couches", 0);

    let mut total_items_count = 0;

    for (name, number) in hashmap.iter() {
        match number {
            0 => println!("Name: {name:?}, Qty: Out of Stock"),
            _ => println!("Name: {name:?}, Qty: {number:?}")
        }
        
        total_items_count = total_items_count + number
    }

    println!("Total Number of Items: {total_items_count:?}")
}

