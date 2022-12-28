fn parse_character(c: &char) -> i32 {
    return match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    };
}
pub fn return_splitted(s: &str) -> i32 {
    let mut current_result: i32 = 0;
    let mut current_i: usize = 0;
    let characters: Vec<char> = s.chars().collect();

    loop {
        let current_value: i32 = match characters.get(current_i) {
            None => break,
            Some(character) => parse_character(character),
        };
        match characters.get(current_i + 1) {
            None => current_result += current_value,
            Some(character) => {
                let next_value = parse_character(character);
                if next_value > current_value {
                    current_result -= current_value
                } else {
                    current_result += current_value
                }
            }
        }
        current_i += 1;
    }
    current_result
}
