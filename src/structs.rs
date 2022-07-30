// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color_2(u8, u8, u8);

struct Person_n {
    first_name: String,
    last_name: String,
}

impl Person_n {
    fn new(first: &str, last: &str) -> Person_n {
        Person_n {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c_1 = Color {
        red: 255,
        green: 125,
        blue: 0,
    };

    c_1.red = 120;
    println!("Color: ({}, {}, {})", c_1.red, c_1.green, c_1.blue);

    let mut c_2 = Color_2(255, 145, 32);
    c_2.2 = 45;
    println!("tuple Color: ({}, {}, {})", c_2.0, c_2.1, c_2.2);

    let mut person_1 = Person_n::new("John", "Doe");
    println!("person is: {}", person_1.full_name());
}
