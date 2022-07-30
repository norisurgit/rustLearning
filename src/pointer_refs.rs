pub fn run() {
    let array_1: [i32; 3] = [1, 2, 3];
    let array_2 = array_1;
    println!("values : {:?}", (array_1, array_2));
    // arrays are primitive.
    // with non-primitive, if you assign another value to a piece of data
    // the first variable will no longer be that value
    // You'll need to use a reference (&) to point to the source

    let vector_1: Vec<i32> = vec![1, 2, 3];
    let vector_2 = &vector_1;
    println!("values : {:?}", (&vector_1, vector_2));
}
