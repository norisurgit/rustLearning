mod new_benchmark_fn;
mod sort_bubble;
fn main() {
    new_benchmark_fn::benchmark_it(|| {
        sort_bubble::bubble_sort(
            [
                25, 328, -85215, -3652, 585, -151, 8521, 0, 126548, -0, 14215, -25, 0, 5, 2205, 8,
            ]
            .to_vec(),
        )
    });
}
