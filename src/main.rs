
fn main() -> () {
    let _num: i64 = 563456000000;
    println!("Max is {}  Min is {}", i64::MAX, i64::MIN);
    if false {
        immutability();
        string();
        tuple();
        destructuring();   
    }
}

fn destructuring() {
    let (first, second, ..) = (1, 2, "another", "value");
    println!("This is value {} {} ", first, second);
 
    let array = [1, 2, 3, 4];
    let [first, second, rest @ ..] = array;
    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(rest, [3, 4]); 
    println!("This is value {:?}", rest);
}


fn tuple() {
    let numbers = (1, 2, "test", "empty");
    // {:?} is debug trait
    println!("This is tuple {:?}", numbers);
    println!("This is tuple {}",  numbers.0);
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
