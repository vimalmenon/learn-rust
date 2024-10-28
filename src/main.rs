
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