
// Main Feature
// - Variable are immutable
// - Value in Heap can only have one owner
// - Value in Stack can only process
// - Heap ownership can be transferred to function


fn main() -> () {
    if false {
        max_values();
        immutability();
        string();
        tuple();
        destructuring();
        stack_string();
        heap_string();
        store_heap_int()
    }
    transfer_ownership()
}


fn transfer_ownership() {
    let value = String::from("testing");
    string_transfer(value);
    // This fails because ownership has been transferred
    // println!("Use this again {}", value);
}

fn string_transfer(value: String) {
    println!("Value transferred to this function {}", value)
}

fn store_heap_int() {
    let heap_int = Box::new(42);
    println!("this is heap int {}", heap_int);
}


fn stack_string() {
    let string = "test";
    let new_string = string;
    println!("this is string {}", string);
    println!("this is new_string {}", new_string);
}

fn heap_string() {
    let string = String::from("This is Viaml Menon");
    // here string is garbage collected
    let new_string = string;
    let stack_string: &str = &new_string;
    // this fail because string memory has been reassigned
    // println!("this is string {}", string);
    // cannot have double pointer
    println!("this is new_string {}", new_string);
    println!("this is stack_string {}", stack_string);
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

fn max_values() {
    println!("i8 : Min is {}  Max is {}", i8::MAX, i8::MIN);
    println!("i8 : Min is {}  Max is {}", i16::MAX, i16::MIN);
    println!("i8 : Min is {}  Max is {}", i32::MAX, i32::MIN);
    println!("i8 : Min is {}  Max is {}", i64::MAX, i64::MIN);
    println!("i8 : Min is {}  Max is {}", i128::MAX, i128::MIN);
    println!("i8 : Min is {}  Max is {}", i128::MAX, i128::MIN);
    println!("u8 : Min is {}  Max is {}", u8::MAX, u8::MIN);
    println!("u16 : Min is {}  Max is {}", u16::MAX, u16::MIN);
    println!("u32 : Min is {}  Max is {}", u32::MAX, u32::MIN);
    println!("u64 : Min is {}  Max is {}", u64::MAX, u64::MIN);
    println!("u128 : Min is {}  Max is {}", u128::MAX, u128::MIN);
}