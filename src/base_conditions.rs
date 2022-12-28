pub fn run() {
    let age: i8 = 18;
    let friend_age: i8 = 24;
    let friend_other_age: i8 = 32;

    if friend_other_age > 21 || friend_age > 18 {
        println!("yeah sure!")
    }

    if age >= 21 && friend_age >= 21 {
        println!("is adult!");
    } else {
        println!("too young :(");
    }

    let is_first_person_adult: bool = if age >= 18 { true } else { false };
    println!("is adult : {}", is_first_person_adult);
}
