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
