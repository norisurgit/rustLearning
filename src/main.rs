mod sort_bubble;
mod sort_insertion;
mod sort_selection;
mod util_new_benchmark_fn;
fn main() {
    let vec_to_be_sorted = [
        2, 02, -52, 8548, -5232, -0, 1254, 0, 3256, 8521, -32, 854, 12, 235, 1487, 9854, 12156,
        -523, 5, 31, 13, 1, 1, 1, 1,
    ]
    .to_vec();
    util_new_benchmark_fn::benchmark_it(
        || sort_insertion::insertion_sort(&vec_to_be_sorted),
        "(insertion sort)",
    );
    util_new_benchmark_fn::benchmark_it(
        || sort_bubble::bubble_sort(&vec_to_be_sorted),
        "(bubble sort)",
    );
    util_new_benchmark_fn::benchmark_it(
        || sort_selection::selection_sort(&vec_to_be_sorted),
        "(selection sort)",
    );
}
