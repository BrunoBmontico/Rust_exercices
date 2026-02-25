fn calculate_fibonacci(number:i32, mut initial_vec:Vec<i32>) -> Vec<i32> {
    let interval:i32 = number;
    let mut first_location:usize = 0;
    let mut second_location:usize = 1;

    for _ in 1..=interval-2 {
        let next:i32 = initial_vec[first_location] + initial_vec[second_location];
        first_location+=1;
        second_location+=1;
        initial_vec.push(next);
    }

    initial_vec
}

pub fn fibonacci() {
    let initial_vec:Vec<i32> = vec![0, 1];
    let fibonacci:Vec<i32> = calculate_fibonacci(15, initial_vec);
    
    println!("{:?}", fibonacci);
}