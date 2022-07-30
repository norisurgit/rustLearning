pub fn run() {
    let mut iteration = 0;
    let mut iteration_2: i32 = 0;
    let numbers_2: [i32; 8] = [4, 5, 6, 8, 5, 4, 7, 8];
    loop {
        iteration += 1;
        println!("iteration : {}", iteration);

        if iteration >= 20 {
            break;
        }
    }

    while iteration_2 <= 100 {
        if iteration_2 % 15 == 0 {
            println!("fizzbuz");
        } else if iteration_2 % 3 == 0 {
            println!("fizz");
        } else if iteration_2 % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", iteration_2);
        }
        iteration_2 += 1;
    }

    for x in numbers_2.iter() {
        println!("{}", x);
    }
}
