// Variables and constants
fn main() {
    // Definition
    let x: i16 = 10;
    println!("x is {x}");

    // Mutability
    let mut y = 5;
    y = 10;

    // Scope
    {
        let z = 50;
    }
    // let s = z;

    // Shadowing
    let t: i32 = 10;
    let t = t + 10;
    println!("t is {t}");

    let u = 3;
    let u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("Inner v is {v}")
    }

    println!("v is {v}");

    // Constants -> must be immutable and defined at compile time and have a defined type
    const MAX_VALUE: u32 = 100;
}

// Types

fn main() {
    // ----- Primite data types ----
    // - Unsigned integers
    let unsigned_num: u8 = 5; // u16, u32, u64, u128

    // - Signed integers
    let signed_num: i8 = 5; // i16, i32, i64, i128

    // - Floating point numbers
    let float_num: f32 = 5.0; // f32, f64

    // - Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    // - Characters
    let char: char = 'a';

    // - Boolean
    let b: bool = true;

    // - Type aliasing
    type Age = u8;
    let peter_age: Age = 42;

    // - Type conversion
    let a: i32 = 10;
    let b: f64 = a as f64;

    // ---- Compound Data Types ----s
    // &str -> string slice - this creates a fixed length string
    let hello_str: &str = "Hello, World!";

    // - String
    let mut flexy_str: String = String::from("Hello, World!");
    flexy_str.push('s');

    // - Arrays
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let num = array[3];
    println!("{:?}", array);
    // Add array with default values, example 0 - 10
    let array_def = [0; 10];
    println!("{:?}", array_def);

    // - Vectors
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // - Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // Accessing tuple
    let first_tuple = tuple.0;
    let tuple_with_str: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");
    // Destructure tuple
    let (x, y, z, s) = tuple_with_str;
    println!("{x} {y} {z} {s}");

    // Unit
    let unit: () = ();
}

// ----- Functions ----
fn main() {
    my_fn("Hello pig");
    let str: &str = "Uhu oho";
    my_fn(str);
    let test = multiplication(2, 3);
    println!("{test}");
    let (addition, subtraction, multiplication) = basic_math(2, 3);
    println!("{addition} {subtraction} {multiplication}");

    // Code block
    let full_name = {
        let first_name = "Nonono";
        let last_name = "Huhuhu";
        format!("{first_name} {last_name}")
    };
}

fn my_fn(s: &str) {
    println!("{s}");
}

// Also works with return
fn multiplication(num1: i32, num2: i32) -> i32 {
    return num1 * num2;
}

// Last code line without ; is returned
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num1 - num2, num1 * num2)
}

// ----- Conditionals ----
fn main() {
    let number = 3;
    if number < 50 {
        println!("{} is less than 50", number);
    } else {
        println!("{} is greater than 50", number);
    }

    let marks = 95;
    // let mut grade = 'N';

    // if marks > 90 {
    //     grade = 'A';
    // } else if marks >= 80 {
    //     grade = 'B';
    // } else if marks >= 60 {
    //     grade = 'C';
    // } else {
    //     grade = 'D';
    // }

    // This can be remade to get captured by a variable
    let grade = if marks > 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 60 {
        'C'
    } else {
        'D'
    };

    println!("Grade: {}", grade);

    let mut grade2 = "F";
    match marks {
        90..=100 => grade2 = "A",
        80..=89 => grade2 = "B",
        60..=79 => grade2 = "C",
        _ => grade2 = "D",
    }

    let grade3 = match marks {
        90..=100 => "A",
        80..=89 => "B",
        60..=79 => "C",
        _ => "D",
    };
    // Alternative approach to store in a variable
}

// ----- Control Flow ----
fn main() {
    // Loops forever unless breaked
    loop {
        println!("Simple loop forever");
        break;
    }

    // Break an outer loop from inside
    'outer: loop {
        println!("Simple loop forever");
        break 'outer;
    }

    // Return from a loop - simplified example without practical use
    let a: i32 = loop {
        break 5;
    };

    // For loop - loop through arrays or vectors
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in vec {
        println!("{i}");
    }

    // While loop
    let mut num: i32 = 0;
    while num < 10 {
        num = num + 1;
    }
}

// ----- Commands/Inputs ----
fn main() {
    let mut n = String::new();
    println!("Input a value:");
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: f64 = n.trim().parse().expect("Invalid input");
    println!("{n}");

    // To improve numerics underscores can be used in large numbers
    let x = 45_000;

    // Static vs constants
    // Constants are inlined and statics are not
    static WELCOME: &str = "Welcome to uwu world";
    const PI: f32 = 3.14;

    // Constants are replaced by their concrete value at compile time
    // So they do not occupy a specific location in memory
    let a = PI;
    let b = PI;

    // Statics do occupy a specific memory location, they are not replaced
    // All instances of the static refer to the same location
    let c = WELCOME;
    let d = WELCOME;
}

// ----- Ownership ----
// 1. Each value has a variable that's its "owner"
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value is dropped
fn main() {
    let s1: String = String::from("world");
    let s2: String = s1;

    // This will throw an error: borrow of moved value
    // -> println!("S1 is {s1}");

    // There are two types of memory
    // 1. Non-Volatile
    // 2. Volatile

    // Non-Volatile
    // refers to hard drive/ssd
    // slow but abundant
    // persists data

    // Volatile
    // refers to RAM/cache
    // fast but not abundant
    // not persisted
    // used during program executions

    // There are three distinct regions in volatile memory
    // Stack, heap, static

    // The stack stores the pointer, len and capactiy of the variable
    // And the pointer points to the location in the heap
    // When a new variable is assigned the value of the old variable
    // a new pointer is created in the stack and the old one is invalidated

    // In cases we want to keep the original we can copy the value and assign it to a new variable
    let s3: String = s2.clone();
    println!("S2 is {s2}");
    println!("S3 is {s3}");
    // This will copy both the stack and the heap adhering to the one owner rule

    // When a variable goes out of scope it's dropped
    // This will avoid memory leaks and dangling pointers

    // This is not true for some primitive types
    // This works because integers, floats, bools and chars are
    // immediately stored on the stack with no reference to the heap
    let x = 15;
    let y = x;
    println!("X is {x} and Y is {y}");
}

// ----- Ownership in Functions ----

fn main() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1);
    // This will produce this error - borrow of moved value
    // -> println!("Vec_1 is {vec_1:?}");
    // Passing a variable to a function has the same effect as assigning the reference to a new variable

    // A function can also give ownership if it returns
    let vec_2 = gives_ownership();
    println!("Vec_2 is {vec_2:?}");

    let vec_3 = takes_and_gives_ownership(vec_2);
    // The function took ownership of vec 2 and gave it to vec 3
    // println!("vec_2 is {vec_2:?}");
    println!("vec_3 is {vec_3:?}");

    let x = 10;
    stack_function(x);
    // Can still access x because float boolin char are copied and not moved
    println!("In main x is {x}");
}

fn takes_ownership(vec: Vec<i32>) {
    println!("Vec is {vec:?}");
}

fn gives_ownership() -> Vec<i32> {
    vec![1, 2, 3]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(4);
    vec
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("In function var is {var}");
}

// ----------- Borrowing -----------
// Borrowing is establishing a reference to some data
// - just like pointers with some rules, does not take ownership

// Rules
// - At any time, you can have either one mutable reference or any number of immutable references
// - References must always be valid

fn main() {
    let mut vec_1 = vec![1, 2, 3];

    // Mutable reference
    let ref1 = &mut vec_1;

    // Gives error because two mutable references are not allowed
    // let ref2 = &mut vec_1;
    // println!("Ref1: {ref1:?}, ref2: {ref2:?}");

    // Immutable reference
    let ref2 = &vec_1;
    let ref3 = &vec_1;
    println!("Ref2: {ref2:?}, ref3: {ref3:?}");

    // Dangling reference
    // let vec_2 = {
    //     let vec_3 = vec![1, 2, 3];
    //     // borrowed value does not live long enough - dangling reference
    //     &vec_3
    // };

    let mut vec_2 = vec![1, 2, 3];
    let ref4 = &vec_2;
    borrows_vec(ref4);
    println!("Ref4: {ref4:?}");
    let ref5 = &mut vec_2;
    takes_and_gives_ownership(ref5);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("Vec is {vec:?}");
}

fn takes_and_gives_ownership(vec: &mut Vec<i32>) {
    vec.push(4);
}

// Won't work because vec will be dropped and its memory will be deallocated so the reference is no longer valid
// fn gives_reference() -> &Vec<i32> {
//     let vec = vec![1, 2, 3];
//     &vec
// }

// ---------- Dereferencing ----------
fn main() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("Some data is {ref_1}, deref_copy is {deref_copy}");

    let mut heap_data = vec![1, 2, 3];
    let ref_2 = &mut heap_data;
    // Heap allocated data cannot be dereferenced in this way becaue assigning heap data means
    // transfering the ownership and the reference doesn't hold the ownership.
    // let deref_copy = *ref_2;
    let deref_copy_2 = ref_2.clone();

    // A mutable reference to some data can be copied only once
    // A mutable reference cannot be copied but only moved
    let move_out = ref_1;
    // use of moved value: `ref_1`
    // let move_out_again = ref_1;
}

// Consider the program below. Which of the following is a correct rust statement that
//  can be used to compare the actual contents that are referenced by variable s4 with the String of "Hello".

//     let s1 = String::from("Hello");
//     let s2 = &s1;
//     let s3 = &s2;
//     let s4 = &s3;

// The variable s2 is single reference.
//The variable s3 is a second order reference and the variable s4 is a third order reference.
// Since adding each star will deref one of the reference so therefore we will use three stars
// to deref the actual contents pointed to by s4.

// -------- Structs and its Types ---------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    let mut my_car = Car {
        owner: String::from("Tom"),
        year: 2020,
        fuel_level: 0.0,
        price: 10_000,
    };

    let car_year = my_car.year;
    my_car.fuel_level = 30.0;

    // While assigning a field of a mutable struct we have to be careful with heap allocated types
    // because their ownership may change
    // let extracted_owner = my_car.owner;
    let extracted_owner = my_car.owner.clone();
    println!("Onwer is: {}", my_car.owner);

    // Sometimes we want to copy most of the fields from an existing struct instance to form a new instance
    let new_car = Car {
        owner: "new_name".to_string(),
        // In this case there are possible partial moves, the heap allocated fields will be moved out of the old instance
        ..my_car
    };

    // Tuple structs
    // Because tuples are flexible there can be some ambigity in the types
    // when we want to use them, so tuple structs are used
    let point_2D = (1, 2);
    let point_3D = (3, 4, 5);

    // Tuple structs can help enforce types for each field
    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    let point1 = Point_2D(1, 2);
    let point2 = Point_3D(3, 4, 5);

    // Unit struct
    // Uncommonm type with no fields
    struct ABC;
}

// -------- Adding functionality to structs ---------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    // Immutable reference self
    fn display_car_info(&self) {
        println!(
            "Car info: Owner: {}, Year: {}, Fuel level: {}, Price: {}",
            self.owner, self.year, self.fuel_level, self.price,
        );
    }

    // Mutable reference self
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    // Own self
    fn sell(self) -> Self {
        self
    }

    // Assosiacted functions / Static functions
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    // An asociated function usually has the name "new" servers often as a constructor
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("Tom"),
        year: 2020,
        fuel_level: 0.0,
        price: 10_000,
    };

    my_car.display_car_info();
    my_car.refuel(5.0);

    let new_owner = my_car.sell();

    let new_car = Car::new("Pigzy".to_string(), 2021);
}

// -------- Enums ---------

enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let mut day = "Satuday".to_string();

    let week_day = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sunday".to_string(),
    ];

    // This requires the use of memory of indexing the vector
    // And the cloned value is also occupying memory
    day = week_day[5].clone();

    // Enums fix this issue
    let day = WeekDay::Saturday;
}

// enum TravelType {
//     Car,
//     Train,
//     Airplane,
// }
// Enums can also hold the data of the enum
enum TravelType {
    Car(f32),
    Train(f32),
    Airplane(f32),
}

// Like structs the functionality on enums can be used with implementation
impl TravelType {
    // fn travel_allowance(&self, miles: f32) -> f32 {
    //     match self {
    //         TravelType::Car => miles * 2.0,
    //         TravelType::Train => miles * 3.0,
    //         TravelType::Airplane => miles * 5.0,
    //     }
    // }

    // When data comes from the enum
    fn travel_allowance(&self) -> f32 {
        match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Airplane(miles) => miles * 5.0,
        }
    }
}
fn main() {
    // let participant = TravelType::Car;
    // println!(
    //     "Allowance of participant is {}",
    //     participant.travel_allowance(100.0)
    // );

    // Declaring data on the enum allows the function to be called without passing the varibale on it, and instead on the enum
    let participant = TravelType::Car(100.0);
    println!(
        "Allowance of participant is {}",
        participant.travel_allowance()
    );
}

// -------- Option ---------

struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        // Using deraf because the student name is behind a ref
        if student.name == *student_name {
            return student.grade;
        }
    }

    None
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(94),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(95),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];

    let student_name = String::from("Bob");
    let grade = get_grade(&student_name, &student_db);

    // match grade {
    //     Some(grade) => println!("Grade: {}", grade),
    //     None => println!("Student not found"),
    // }

    // If not interested in both None and Some outcome use if let

    if let Some(g) = grade {
        println!("Grade: {}", g);
    }
}

// Rust does not define null, however we have the Option enum which offers None
// enum Option<T> {
//     Some(T),
//     None
// }

// --------  Result ---------

// Result enum is good for returning errors
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        // Using deraf because the student name is behind a ref
        if student.name == *student_name {
            return student.grade;
        }
    }

    None
}

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(());
        }
    }

    Err(String::from("Failed to find student"))
}

fn find_grade_by_student_name(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }

    Err(String::from("Failed to find student"))
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(94),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(95),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];

    let student_name = String::from("Bob");

    // match grade {
    //     Some(grade) => println!("Grade: {}", grade),
    //     None => println!("Student not found"),
    // }

    // If not interested in both None and Some outcome use if let

    // let student_status = check_student(&student_name, &student_db);

    // match student_status {
    //     Ok(_) => {
    //         let grade = get_grade(&student_name, &student_db);
    //         if let Some(g) = grade {
    //             println!("Grade: {}", g);
    //         }
    //     }
    //     Err(errr) => println!("{}", errr),
    // }

    let student_status = find_grade_by_student_name(&student_name, &student_db);

    match student_status {
        Ok(grade) => {
            if let Some(g) = grade {
                println!("Grade: {}", g);
            }
        }
        Err(errr) => println!("{}", errr),
    }
}

// --------  Hashmap ---------
use std::collections::HashMap;

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Tom", 40);
    person.insert("Bob", 50);
    person.insert("Sam", 60);

    println!("Person: {:?}", person);
    println!("The age is: {:?}", person.get("Tom").unwrap());

    if person.contains_key("Tomy") {
        println!("Exists");
    } else {
        println!("Doesn't exist");
    }

    match person.get("Tom") {
        Some(value) => println!("Value: {value}"),
        None => println!("Not found"),
    }

    for (name, age) in &person {
        println!("Name: {name}, Age: {age}");
    }

    let mut likes: HashMap<&str, &str> = HashMap::new();
    // Insert overwrites the previous value
    // likes.insert("Tom", "apple");
    // likes.insert("Tom", "banana");
    // println!("Likes: {:?}", likes);

    // Entry doesn't overwrite the previous value
    likes.entry("Tom").or_insert("apple");
    likes.entry("Tom").or_insert("banana");
    println!("Likes: {:?}", likes);

    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("Freq: {:?}", freq_vec);
}

// --------  Modules ---------

use main::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, String::from("John"), String::from("VwqzK@example.com"));
    let order = Order::new(1, product, customer, 2);

    println!("Total cost of ther order: ${}", order.total_bill());
}

// ------- LIB Modules -------
// Re-exporting without having to declare the whole module public
pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};

mod product {
    // Bringing items into scope
    pub use category::Category;

    // Making the struct public doesn't make it's fields public
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }
    }

    // Unlike structs making enums public will make it's values public as well
    mod category {
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price * self.calculate_tax()
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;

    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}

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

//! # Online Business
//! This is a rust library for online store

// ---------- Modules -----------
// Re-exporting without having to declare the whole module public
pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};

mod product {
    // Bringing items into scope
    pub use category::Category;
    /// Struct for storing product related information
    #[derive(PartialEq, Debug)]

    // Making the struct public doesn't make it's fields public
    pub struct Product {
        id: u64,
        pub name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        /// # Example
        /// ```
        /// use project_1234::Category;
        /// use project_1234::Product;
        /// let some_product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
        /// assert_eq!(some_product.name, String::from("Laptop"));
        /// ```
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }
    }

    // Unlike structs making enums public will make it's values public as well
    mod category {
        /// Enum for representing product category
        #[derive(PartialEq, Debug)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price * self.calculate_tax()
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;

    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}

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

// ------------- Traits ----------------

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) {
//         println!("The area is: {}", self.length * self.width);
//     }
// }

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter is not implemented!");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("The area of the rectangle is: {area_of_rect}");
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("The perimeter of the rectangle is: {perimeter_of_rect}");
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("The area of the square is: {area_of_square}");
        area_of_square
    }
}

fn main() {
    let r = Rectangle {
        width: 5.0,
        length: 3.0,
        line_width: 1,
        color: String::from("red"),
    };

    let s = Square {
        side: 5.0,
        line_width: 1,
        color: String::from("blue"),
    };

    r.area();
    s.area();

    r.perimeter();
    s.perimeter();
}

// Link list 2

#[derive(Debug)]
struct LinkList {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

impl LinkList {
    fn new() -> LinkList {
        LinkList { head: None }
    }

    fn add(&mut self, element: i32) {
        // cannot move out of `self.head` as enum variant `Some` which is behind a mutable reference
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     Some(previous_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));

        //         self.head = new_node;
        //     }
        // }

        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));

        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;

        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let mut list = LinkList::new();

    list.add(5);
    list.add(7);
    list.add(10);

    println!("List: {:?}", list);
    println!("{}", list.remove().unwrap());
}

// Doubly Link List

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug)]
struct Doubly_LinkList {
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
    prev: pointer,
}

type pointer = Option<Rc<RefCell<Node>>>;

impl Doubly_LinkList {
    fn new() -> Self {
        Doubly_LinkList {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so there's nothing to remove");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    let mut list1 = Doubly_LinkList::new();

    list1.add(30);
    list1.add(20);
    list1.add(34);
}
