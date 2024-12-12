// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Green,
    Blue,
    Yellow,
}

struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn create_new_box() -> Self {
        Self {
            dimensions: (32.0, 20.2, 56.0),
            weight: 45.8,
            color: BoxColor::Red
        }
    }

    fn print_characteristics(&self) {
        let (length, breadth, height) = self.dimensions;

        println!("Dimensions: {:?}, {:?}, {:?}", length, breadth, height);
        println!("Weight: {:?}", self.weight);
        println!("Color: {:?}", match self.color {
            BoxColor::Blue => "blue",
            BoxColor::Yellow => "yellow",
            BoxColor::Green => "green",
            BoxColor::Red => "red",
        })
    }
}

fn main() {
    let my_box = ShippingBox {
        dimensions: (10.0, 23.8, 40.8),
        weight: 52.5,
        color: BoxColor::Blue
    };

    my_box.print_characteristics();

    let new_box = ShippingBox::create_new_box();   

    new_box.print_characteristics();
}

