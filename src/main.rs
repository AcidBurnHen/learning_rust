// --------  Modules ---------

use array_tool::vec::*;
use main_test_123::{Category, Customer, Order, Product};

fn main() {
    let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let product2 = Product::new(1, String::from("T-Shirt"), 20.99, Category::Clothing);
    let product3 = Product::new(1, String::from("Book"), 20.99, Category::Books);

    let set1: Vec<&Product> = vec![&product1, &product2];
    let set2: Vec<&Product> = vec![&product2, &product3];

    let intersection = set1.intersect(set2);
    println!("Intersection: {:?}", intersection);
}
