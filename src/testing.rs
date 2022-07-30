pub fn run() {
    let mut my_name = String::from("Abdennour");

    my_name.push_str(" is haha");
    println!("{:?}", my_name);

    struct Person {
        a_name: String,
        a_family_name: String,
        an_age: i32,
        is_alive: bool,
    }

    let person_1 = Person {
        a_name: String::from("Oussama"),
        a_family_name: String::from("Megzi"),
        an_age: 15,
        is_alive: true,
    };
}
