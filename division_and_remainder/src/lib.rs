pub fn divide(x: i32, y: i32) -> (i32, i32) {

    let first = x / y;
    let second = x % y;
    return let tup: (i32, i32) = (first, second);
}