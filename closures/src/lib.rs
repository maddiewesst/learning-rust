pub fn first_fifty_even_square() -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    let mut num: i32 = 2;
    while result.len() != 50 {
            result.push(num.pow(2));
            num += 2;
        }
    return result;
}
    
    
    
    // pub fn first_fifty_even_square() -> Vec<i32> {
    //     let mut vec = Vec::new();
    //     for x in 2..=100 {
    //         if x % 2 == 0 {
    //             vec.push(x * x);
    //         }
    //     }
    //     vec
    // }
    
    
    
    // fn first_fifty_even_square() -> Vec<i32> {
    //     let mut result = vec![];
    //     let mut count = 0;
    //     let mut current_num = 0;
        
    //     let even_numbers = || {
    //         current_num += 2;
    //         Some(current_num)
    //     };
        
    //     let even_squares = even_numbers.filter(|&n| n % 2 == 0).map(|n| n * n);
        
    //     while count < 50 {
    //         if let Some(num) = even_squares.next() {
    //             result.push(num);
    //             count += 1;
    //         }
    //     }
        
    //     result
    // }
// }