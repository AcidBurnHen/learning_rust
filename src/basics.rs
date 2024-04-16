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
