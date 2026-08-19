fn main() -> () {
    let _num: i64 = 563456000000;
    println!("Max is {}  Min is {}", i64::MAX, i64::MIN);
    immutability();
    string();
}



// ## this code will fail because of immutability
// fn fail_immutability() {
//     let value = "test";
//     println!("This is value {} ", value);
//     value = "test 1";
//     println!("This is value {} ", value);
// }

fn immutability() {
    let mut value = "test";
    println!("This is value {} ", value);
    value = "test 1";
    println!("This is value {} ", value);
}


fn string() {
    // This value is stored in readonly date on memory and not stack
    // This string value cannot be changed
    let stack_value: &str = "This is String from stack";
    println!("Value is {}", stack_value);

    // I can increase and decrease the size of string as it's in HEAP
    let heap_value: String = String::from("This is String heap");
    println!("Value is {}", heap_value);

    // This demonstrated idea of mutability
    let mut new_heap_value: String = String::from("This is String heap");
    println!("Value is {}", new_heap_value);
    new_heap_value.push_str(" : More Value");
    println!("Value is {}", new_heap_value);
}
