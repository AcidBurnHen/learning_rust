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
}

fn takes_ownership(vec: Vec<i32>) {
    println!("Vec is {vec:?}");
}

fn gives_ownership() -> Vec<i32> {
    vec![1, 2, 3]
}
