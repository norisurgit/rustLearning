pub fn go() {
    let mut names: [i32; 4] = [2, 5, 8, 4];

    names[3] = 8;

    println!("{:?}", names);

    println!("{}", names[0]);

    println!("{}", names.len());
}
