use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let mean: f64 = sum as f64 / list.len() as f64;
    return mean
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut val = 0;
    let mut n = 0;
    let mut sorted = list.clone();
    sorted.sort();
    if sorted.len() % 2 == 1 { 
        val = sorted.len() / 2; 
        n = sorted[val];
    }else{
        val = sorted.len() / 2;
        n += sorted[val];
        n += sorted[val-1];
        n /= 2;
    }
    n
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for item in list {
        *map.entry(item).or_insert(0) += 1;
    }
    return bigger(map)
}

pub fn bigger(h: HashMap<&i32, i32>) -> i32 {

    let mut bv: i32 = 0;
    for (_, value) in h.iter() {
        if value > &bv {
            bv = *value
        }
    }
    bv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
