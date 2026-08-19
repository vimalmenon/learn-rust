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
    let stack_value: &str = "This is String from stack";
    println!("Value is {}", stack_value);

    let heap_value: String = String::from("This is String heap");
    println!("Value is {}", heap_value);
}
