pub fn insertion_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut iteration: usize = 0;
    let mut arr_copied = arr.clone();
    while iteration < arr_copied.len() {
        let mut iteration_2: usize = iteration;
        while iteration_2 > 0 {
            let preceding: i32 = arr_copied[iteration_2 - 1];
            if preceding > arr_copied[iteration_2] {
                arr_copied[iteration_2 - 1] = arr_copied[iteration_2];
                arr_copied[iteration_2] = preceding;
            }
            iteration_2 -= 1;
        }
        iteration += 1;
    }
    println!("Insertion sorted: {:?}", arr_copied);
    return arr_copied;
}
