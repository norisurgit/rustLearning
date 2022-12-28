pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut is_sorted: bool = false;
    let mut iterator: usize = 0;
    let mut should_repeat = false;
    println!("original: {:?}", arr);
    while !is_sorted {
        if iterator < arr.len() - 1 {
            if arr[iterator] > arr[iterator + 1] {
                let current = arr[iterator];
                arr[iterator] = arr[iterator + 1];
                arr[iterator + 1] = current;
                should_repeat = true;
            }
            iterator += 1
        } else if iterator >= arr.len() - 1 && should_repeat {
            iterator = 0;
            should_repeat = false;
        } else {
            is_sorted = true;
            println!("Bubble sorted: {:?}", arr);
        }
    }
    return arr;
}
