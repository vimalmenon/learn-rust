# Learn Rust

## Run Rust script
```sh
cargo run
```

### Create Release
```sh
cargo run --release
```

### Example

#### Simple Program
```rs
fn main() {
    println!("Hello, world!");   
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
