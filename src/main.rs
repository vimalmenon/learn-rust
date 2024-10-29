mod first_script;
use first_script::test;

fn main() {
    let mut value = String::from("This is Text, ");
    println!("Text : {}", value);
    println!("Memory : {:p}", &value);
    value.push_str("test");
    println!("Text : {}", value);
    println!("{}", test());
}