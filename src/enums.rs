enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // perform action depending on info
    match m {
        Movement::Up => println!("avatar moving up!"),
        Movement::Down => println!("avatar moving down!"),
        Movement::Left => println!("avatar moving left!"),
        Movement::Right => println!("avatar moving right!"),
    }
}

pub fn run() {
    let avatar_1 = Movement::Left;
    let avatar_2 = Movement::Right;
    let avatar_3 = Movement::Up;
    let avatar_4 = Movement::Down;

    move_avatar(avatar_1);
    move_avatar(avatar_2);
    move_avatar(avatar_3);
    move_avatar(avatar_4);
}
