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

#### Example or simple rust
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