use std::{fs::File, io::Write};

#[allow(unused, dead_code)]



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
}

