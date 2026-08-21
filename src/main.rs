use std::{fs::File, io::Write};

#[allow(unused, dead_code)]

struct Person {
    name: String,
    age: u32,
}

trait PersonTrait {
    fn new(name: String, age: u32) ->Person;
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}

impl PersonTrait for Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}


fn main() -> () {
    let file = File::create("text.csv");
    match file {
        Ok(mut file) => {
            file.write(b"1, 'Vimal Menon'").unwrap();
            file.write(b"\n2, 'John Doe'").unwrap();
        },
        Err(e) => println!("Error creating file: {}", e),
    }

    let items = vec![1, 2, 3, 4, 5];

    let doubled_items: Vec<i32> = items.iter().map(|x: &i32| x * 2).collect();
    println!("{:?}", doubled_items);
    println!("{:?}", items);

    let person = Person::new("Vimal Menon".to_string(), 25);
    person.get_name();
    person.get_age();

}