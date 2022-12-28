use std::mem;

pub fn run() {
    let small_number: i32 = 45;
    println!("variable is: {}", small_number);

    const IS_ACTIVE: bool = true;
    assert_eq!(true, IS_ACTIVE);
    println!("{:?}", (IS_ACTIVE));

    let person: (&str, &str, i8) = ("Abdennour", "Bnsekrane", 25);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    println!(
        "person is taking {} bytes of memory",
        mem::size_of_val(&person)
    );
}
