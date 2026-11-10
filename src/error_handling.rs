pub fn run_tests() {
    match safe_divide(10, 2) {
        Ok(value) => {println!("Result: {}", value)}
        Err(message) => {println!("Error: {}", message)}
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Divide by zero")
    } else {
        Ok(a / b)
    }
}
