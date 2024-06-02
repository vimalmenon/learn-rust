fn main()  {
    println!("Hello, world!");
    let another_str = "test";
    let mut string:String = String::from("Vimal Menon");
    let new_string  = String::new();
    let number :u8= 5;
    let value: u8 = add(&number);
    println!("{value} {} {} {} {new_string}", string, number, another_str);

    string.push_str("Value");
}

fn add(number:&u8) -> u8 {
    println!("Adding number {:p}",number);
    number + 5
}

