use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let mean: f64 = sum as f64 / list.len() as f64;
    return mean
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut val;
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
    let mut map: HashMap<String, u32> = HashMap::new();

    for n in list.iter() {
        if map.contains_key(&n.to_string()) {
            *map.entry(n.to_string()).or_insert(0) += 1;
            continue;
        }
        map.insert(n.to_string(), 1);
    }
    let str: &String = map.iter().max_by_key(|entry| entry.1).unwrap().0;
    str.parse().unwrap()
}

