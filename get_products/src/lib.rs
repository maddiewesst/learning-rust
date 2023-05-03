pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    let mut result = vec![1; n];
    let mut left_product = 1;
    let mut right_product = 1;

    if arr.len() == 1 {
        return vec![];
    }

    for i in 0..n {
        result[i] *= left_product;
        left_product *= arr[i];

        result[n - 1 - i] *= right_product;
        right_product *= arr[n - 1 - i];
    }

    result

}

