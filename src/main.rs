use std::io::stdin;

fn main() {
    let mut input = String::new();
    println!("Please enter a text");
    stdin().read_line(&mut input).expect("Please enter the correct number");
    println!("Text you entered is {}", input);
}
