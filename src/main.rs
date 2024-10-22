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
