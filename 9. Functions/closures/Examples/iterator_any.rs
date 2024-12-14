// Iterator::find is a function which iterates over an iterator and searches for the first value which satisfies some condition.
// If none of the values satisfy the condition, it returns None
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vectors yields `&i32`. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` for vectors yields `&i32`. NO destructure to `i32`
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}
