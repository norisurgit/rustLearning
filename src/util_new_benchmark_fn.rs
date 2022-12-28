use std::time::SystemTime;
pub fn benchmark_it<F: Fn() -> T, T>(f: F, fn_type: &str) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took: {:?} to execute {}\n", duration, fn_type);
    return result;
}
