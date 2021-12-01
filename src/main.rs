use std::fs;

fn main() {
    let input = fs::read_to_string("input/example").expect("Error while reading");
    println!("{}", input);
}
