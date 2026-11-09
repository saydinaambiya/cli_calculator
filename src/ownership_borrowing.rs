
pub fn run_tests() {
    let word = String::from("hello");
    let length= calculate_length(&word);
    println!("word: {}, length: {:?}", word, length);
}

fn calculate_length(word: &str) -> usize {
    word.len()
}
