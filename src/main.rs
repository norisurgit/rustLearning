mod leet_roman_to_int;
fn main() {
    let string_for_number = String::from("CD");
    let number_converted = leet_roman_to_int::return_splitted(&string_for_number);

    println!("result: {}", number_converted);
}
