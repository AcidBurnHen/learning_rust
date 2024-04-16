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
