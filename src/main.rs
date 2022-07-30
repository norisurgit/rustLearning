use std::env::*;
fn main() {
    for argument in args() {
        println!("{argument}")
    }
}
