use std::io;

fn vogals_count(input:&str) -> i32 {
    let mut count:i32 = 0;

    for vogal in input.chars() {
        if "aeiou".contains(vogal) {
            count+=1;
        }
    }
    count
}

fn read_input() -> String {
    let mut value:String = String::new();
    io::stdin().read_line(&mut value).expect("Error reading input...");
    value.trim().to_string()
}

pub fn vogals() {
    let word:String = read_input();
    let result:i32 = vogals_count(&word);

    println!("{}", result);
}