pub fn bubble_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut is_sorted: bool = false;
    let mut iterator: usize = 0;
    let mut should_repeat = false;
    let mut arr_copied = arr.clone();
    while !is_sorted {
        if iterator < arr_copied.len() - 1 {
            if arr_copied[iterator] > arr_copied[iterator + 1] {
                let current = arr_copied[iterator];
                arr_copied[iterator] = arr_copied[iterator + 1];
                arr_copied[iterator + 1] = current;
                should_repeat = true;
            }
            iterator += 1
        } else if iterator >= arr_copied.len() - 1 && should_repeat {
            iterator = 0;
            should_repeat = false;
        } else {
            is_sorted = true;
            println!("Bubble sorted: {:?}", arr_copied);
        }
    }
    return arr_copied;
}
