fn fatorial(mut number:i32) -> i32 {
    for i in (1..number).rev() {
        number = number * i;
    }
    number
}

fn recurcao(fatorial:i32) -> i32 {
    let mut result = fatorial;
    let mut i = 2;

    while result > 1 {
        result = result / i;
        i+=1
    }
    i-1
}

pub fn fatorial_recurcao() {
    let number:i32 = 4;

    let fatorial:i32 = fatorial(number);
    let recurcao:i32 = recurcao(fatorial);
    
    println!("Fatorial: {}", fatorial);
    println!("Recurcao: {}", recurcao);
}