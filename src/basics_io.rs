use std::io;

pub fn run_calculator() {
    println!("=== Rusty Calculator ===");
    println!("Enter the first number:");

    // create a mutable String variable to store the input.
    let mut first_number = String::new();
    let mut second_number = String::new();

    // read input from the standard input (keyboard)
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line"); // coarse error handling instead of try-catch

    // HINT 1: parse the String into a number
    let parsed_first_number: i32 = first_number.trim().parse().expect("Not an integer");

    // HINT 2: ask for the second Number
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    // HINT 3: parse the second number
    let parsed_second_number: i32 = second_number.trim().parse().expect("Not an integer");

    // HINT 4: calculate and print
    let result: i32 = parsed_first_number + parsed_second_number;
    println!("The result is {}", result);
}