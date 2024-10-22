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