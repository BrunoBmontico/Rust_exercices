fn calculate_even_odd(number:i32) -> &'static str {
    match number % 2 == 0 {
        0 => "EVEN",
        _ => "ODD",
    }
}

pub fn even_odd() {
    let number:i32 = 5;

    let result:&str = calculate_even_odd(number);

    println!("{}", result);
}