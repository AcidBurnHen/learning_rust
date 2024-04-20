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
