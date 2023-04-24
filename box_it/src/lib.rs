

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s
        .split(' ')
        .map(|num_str| {
            let mut num = num_str.trim_end_matches('k').parse::<f32>().unwrap();
            if num_str.ends_with('k') {
                num *= 1000.0;
            }
            num as u32
        })
        .collect();
    Box::new(nums)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}