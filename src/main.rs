
fn main() {
    let mut some_value:String = String::from("Vimal Menon");
    some_fn(&mut some_value);
    println!("{}", some_value);
}

fn some_fn(value: &mut String) {
    value.push_str(" - This is added from some_fn");
}