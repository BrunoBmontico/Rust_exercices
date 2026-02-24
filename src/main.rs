fn multiple_3(number:i32) -> Option<&'static str> {
    if number % 3 == 0 {
        Some("FIZZ")
    } else {
        None
    }
}

fn multiple_5(number:i32) -> Option<&'static str> {
    if number % 5 == 0 {
        Some("BUZZ")
    } else {
        None
    }
}

fn main() {
    for number in 1..=100 {
        if let Some(result_1) = multiple_3(number) && let Some(result_2) = multiple_5(number){
            println!("{}: {}{}", number, result_1, result_2)
        } else if let Some(result) = multiple_3(number) {
            println!("{}: {}", number, result);
        } else if let Some(result) = multiple_5(number) {
            println!("{}: {}", number, result);
        }
    }
}
