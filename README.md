# Learn Rust

## Run Rust script
```sh
cargo run
```

### Create Release
```sh
cargo run --release
```

### Examples

#### Simple Program
```rs
fn main() {
    println!("Hello, world!");   
}
```

#### Get User Input
```rs
use std::io::stdin;

fn main() {
    let mut input = String::new();
    println!("Please enter a text");
    stdin().read_line(&mut input).expect("Please enter the correct number");
    println!("Text you entered is {}", input);
}
```
#### Example of mutation and reference
```rs
fn main() {
    let mut some_value:String = String::from("Vimal Menon");
    some_fn(&mut some_value);
    println!("{}", some_value);
}

fn some_fn(value: &mut String) {
    value.push_str(" - This is added from some_fn");
}
```
#### Example of list / Vector
```rs
fn main() {
    // Fix in size, so push don't work
    let a = ["a", "b"];
    println!("{:?}", a);

    // Resizable List, this needs to be mutable as it need to increase in size.
    let mut v1  = Vec::new();
    v1.push("test1");
    v1.push("test2");
    println!("{:?}", v1);

    // List with initial values
    let mut v2 = vec!["test1", "test2"];
    v2.push("test3");
    println!("{:?}", v2);
}
```
#### Example of Borrowing
```rs
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
```
##### Example for Loop
```rs
fn main() {
    let mut items = [String::from("One"), String::from("Two"), String::from("Three")];
    simple_loop(&mut items);
    for_loop(&mut items);
    while_loop(&mut items);
}
fn simple_loop(items: &mut [String; 3]) {
    let mut counter = 0;
    loop {
        if counter == items.len() {
            break;
        }
        items[counter].push_str(" - simple loop");
        println!("Simple Loop : {}", items[counter]);
        counter += 1;
    }
}
fn for_loop(items: &mut [String; 3]) {
    for item in items.iter_mut() {
        item.push_str(" - for loop");
        println!("For Loop: {}", item);
    }
}

fn while_loop(items: &mut [String; 3]) {
    let mut counter = 0;
    while counter < items.len() {
        items[counter].push_str(" - while loop");
        println!("While Loop: {}", items[counter]);
        counter += 1;
    }
}
```
#### Example of String
```rs
fn main() {
   str_fn();
   string_fn();
}

fn str_fn() {
    let test: &str = "Vimal";
    println!("test : {}", test);
    let test1 = test.to_string();
    println!("test1 : {}", test1);
    let test2 = test;
    println!("test : {}, test1 : {}, test2 : {}",test, test1, test2);
    let test3 = test1.clone();
    println!("test : {}, test1 : {}, test2 : {}, test3 : {}",test, test1, test2, test3);
}

fn string_fn() {
    let test = String::from("Vimal");
    println!("test : {}", test);
    let test1 = test.clone();
    println!("test : {}, test1 : {}",test, test1);
    let test2 = test.to_owned();
    println!("test : {}, test1 : {}, test2 : {}",test, test1, test2);
    /* 
    This will failed because test1 is moved to test2
    let test3 = test1;
    println!("test : {}, test1 : {}, test2 : {}, test3 : {}",test, test1, test2, test3);
    */
}
```
```rs
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
```
#### Findings
- &str, u8 & bool type can be referenced to another variable without any error
