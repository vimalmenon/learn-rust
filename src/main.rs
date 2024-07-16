// Import the test function from first_script.rs
mod first_script;
use first_script::test;

fn main()  {
    // println!("Hello, world!");
    // let another_str = "test";
    // let mut string:String = String::from("Vimal Menon");
    // let new_string: String  = String::new();
    // let number :u8= 5;
    // let value: u8 = add(&number);
    // println!("{value} {} {} {} {new_string}", string, number, another_str);

    // string.push_str("Value");
    add(5);
    array();
    println!("{:?} this is test", test());
}

fn add(number:u8) -> u8 {
    println!("Adding number {}",number);
    number + 5
}

fn array() {
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}", numbers);
}
