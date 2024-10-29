
fn main() {
    let mut value = String::from("This is Text, ");
    println!("Text : {}", value);
    println!("Memory : {:p}", &value);
    value.push_str("test");
    println!("Text : {}", value);
}
