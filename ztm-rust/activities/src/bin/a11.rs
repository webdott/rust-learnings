// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item

// Notes:
// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_id(item: &GroceryItem) {
    println!("id = {:?}", item.id);
}

// * Create a function to display the id number, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("Quantity = {:?}", item.quantity);
}

fn main() {
    let milk = GroceryItem{
        id: 123,
        quantity: 100,
    };

    display_id(&milk);
    display_quantity(&milk);
}

