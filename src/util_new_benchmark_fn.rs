use std::time::SystemTime;
pub fn benchmark_it<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took: {:?} to bubble sort", duration);
    return result;
}
