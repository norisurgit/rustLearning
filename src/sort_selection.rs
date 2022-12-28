pub fn selection_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr_copied: Vec<i32> = arr.clone();
    let mut iteration: usize = 0;
    while iteration < arr_copied.len() - 1 {
        let mut iteration_2 = iteration;
        let mut smallest = arr_copied[iteration_2];
        let mut smallest_index = iteration;
        while iteration_2 < arr_copied.len() {
            if arr_copied[iteration_2] < smallest {
                smallest = arr_copied[iteration_2];
                smallest_index = iteration_2;
            }
            iteration_2 += 1;
        }
        let current = arr_copied[iteration];
        arr_copied[iteration] = smallest;
        arr_copied[smallest_index] = current;
        iteration += 1;
    }
    println!("Selection sorted: {:?}", arr_copied);
    return arr_copied;
}
