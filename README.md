# Learn Rust

- [ ] Basics of Rust
- [ ] If / else 
- [ ] Loops
- [ ] Function
- [ ] Ownership and Reference
- [ ] Mutation
- [ ] Struts
- [ ] Enums
- [ ] Traits
- [ ] Implementation
- [ ] Vector, Hashmap
- [ ] Module and Library
- [ ] Error Handling
- [ ] Result
- [ ] Generics
- [ ] Life Time
- [ ] Sample code
    - [ ] Reading the file
    - [ ] Traversing the file system
    - [ ] Random Number
    - [ ] DateTime
    - [ ] Other common Libraries
- [ ] Web Framework
- [ ] BlockChain (Solana)
- [ ] Async / Await / Futures

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
    let val1 = 5;
    let val2 = 10;
    println!("i32 : {}", test_num(val1, val2));

    let bool = false;
    println!("bool : {}", test_bool(bool));

    let char = 'a';
    println!("char : {}", test_char(char));

    let str = "Hello, World!";
    println!("str : {}", test_str(str));
}

fn test_num(val1: i32, val2: i32)-> i32 {
    val1 + val2 + 5
}
fn test_bool(val: bool) -> bool {
    !val
}
fn test_char(val: char) -> char {
    val
}
fn test_str(val: &str) -> &str {
    val
}
```

#### Example Of Reference
```rs
fn main() {
    let mut value = String::from("This is Text, ");
    println!("Text : {}", value);
    println!("Memory : {:p}", &value);
    value.push_str("test");
    println!("Text : {}", value);
}
```
#### Example getting function from another file
```rs
// first_script.rs
pub fn test() -> u8 {
    println!("This is a test function");
    5
}

//main.rs
mod first_script;
use first_script::test;

fn main() {
    println!("{}", test());
}
```

#### Example of import from file
```rs
use std::fs;

fn main() {
    let result = fs::read_to_string("./src/first_script.rs");
    println!("{:?}", result);
}
```