// Import the test function from first_script.rs
mod first_script;
use first_script::test;

#[derive(Debug)]
enum Gender {
    Male,
    Female
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    gender: Gender
}

fn main()  {
    // println!("Hello, world!");
    // let another_str = "test";
    // let mut string:String = String::from("Vimal Menon");
    // let new_string: String  = String::new();
    // let number :u8= 5;
    // let value: u8 = add(&number);
    // println!("{value} {} {} {} {new_string}", string, number, another_str);

    // string.push_str("Value");
    let mut number:u8 = 5;
    number = add(number);
    array();
    println!("{:?} this is test", test());
    println!("{:?}", number);
    let vimal_menon = User { name: String::from("Vimal Menon"), age: 40, gender: Gender::Male };
    println!("{:?}", vimal_menon);
}

fn add(number:u8) -> u8 {
    println!("Adding number {}",number);
    number + 5
}

fn array() {
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}", numbers);
} 
