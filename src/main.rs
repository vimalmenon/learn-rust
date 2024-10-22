fn main() {
    let mut string = String::from("This is string");
    println!("This is Before, {}", string);
    borrowed(&mut string);
    string.push_str(", This is after borrowed");
    println!("This is After, {}", string);
}

fn borrowed(string: &mut String) {
    string.push_str(", This is inside borrowed");
    println!("This is Inside, {}", string);
}