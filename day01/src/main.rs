use std::fs;

fn main() {
    let input = read_input();
}


// Read all content of the file called 'input'
fn read_input() -> String {
    return fs::read_to_string("input").expect("Failed to read file");
}