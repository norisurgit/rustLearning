pub fn run() {
    let mut my_vector: Vec<&str> = vec!["Nori", "Nono", "Abdennour"];

    my_vector[2] = "Karita";

    my_vector.push("Abdennour");

    my_vector.pop();

    for x in my_vector.iter() {
        println!("name is : {}", x);
    }

    println!("vector: {:?}", my_vector)
}
