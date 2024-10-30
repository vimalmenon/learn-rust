use std::fs;

fn main() {
    let result = fs::read_to_string("./src/first_script.rs");
    println!("{:?}", result);
}
