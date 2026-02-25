pub fn first_numbers() {
    let interval:i32 = 10;
    let mut fibonacci:Vec<i32> = vec![0, 1];
    let mut first_location:usize = 0;
    let mut second_location:usize = 1;
    
    for _ in 1..=interval {
        let next:i32 = fibonacci[first_location] + fibonacci[second_location];
        first_location+=1;
        second_location+=1;
        fibonacci.push(next);
    }

    println!("{:?}", fibonacci)
}