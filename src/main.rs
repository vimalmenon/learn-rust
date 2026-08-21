#[derive(Debug)]
#[warn(unused)]
#[warn(dead_code)]

struct Person {
    name: String,
    age: u8,

}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn say_name(&self) -> Option<&str>{
        println!("My name is {}", self.name);
        return None;
    }
    
}


struct Example {
    name: String,
}


trait ExampleTrait {
    fn new(name: String) -> Self;
    fn say_name(&self) -> Option<&str>;
    
}

impl ExampleTrait for Example {
    fn new(name: String) -> Self {
        Self { name }
    }
    fn say_name(&self) -> Option<&str> {
        println!("My name is {}", self.name);
        return None;
    }    
}

fn main() -> () {
    let example = Example::new(String::from("Bob"));
    example.say_name();
}


