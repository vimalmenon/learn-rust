
fn main() {
    let some_str: &str = "vimal";
    let another_str = some_str;
    println!("{}, {}", some_str, another_str);
    let some_number:u8 = 200;
    let another_number = some_number;
    println!("{}, {}", some_number, another_number);
    let some_bool:  bool = true;
    let another_bool = some_bool;
    println!("{}, {}", some_bool, another_bool);
    // let some_string: String = String::from("Vimal");
    // let another_string = some_string;
    // println!("{}, {}", some_string, another_string);
}
