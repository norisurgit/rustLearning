pub fn run() {
    salute("Ello", "govenor");

    let get_sum = add(5, 12);
    println!("sum : {}", get_sum);

    let n3: i32 = 15;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // closures (kind of like functions) but can use scoped variables
    println!("Closure sum : {}", add_nums(4, 5));
}

fn salute(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
