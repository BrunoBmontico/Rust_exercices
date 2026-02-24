fn fatorial(mut number:i32) -> i32 {
    for i in (1..number).rev() {
        number = number * i;
    }
    number
}

pub fn fatorial_recurcao() {
    let number:i32 = 4;

    let result:i32 = fatorial(number);
    
    println!("{}", result);
}