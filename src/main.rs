// ------------- Generics ----------------
// Generics allow us to define fns, structs, enums and traits with placeholders for datatypes enabeling flexible and re-usable code
// To-be specified letter types are called generic parameters
// They allow the support a variety of datatypes
// Generics are defined with angle brackets < >

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!("x: {} , y: {}", self.x, self.y);
    }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!("x: {} , y: {}", self.x, self.y);
    }
}

// Free function example using generics
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn add_points_i32(p1: &Point<i32, i32>, p2: &Point<i32, i32>) -> Point<i32, i32> {
    unimplemented!();
}

fn add_points_f64(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> Point<f64, f64> {
    unimplemented!();
}

fn main() {
    let origin = Point::new(0, 0);
    let pi = Point::new(2.0, 1.2);
    let p2 = Point::new(1, 2.9);

    origin.printing();

    add_points(&origin, &origin);
}
