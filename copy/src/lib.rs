pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let mut f = 0.0;
    f += f64::from(c);
    let abs = (c.abs() as f64).ln();
    (c, f.exp(), abs)
}

pub fn str_function(a: String) -> (String, String) {

   let result = a.split_whitespace()
    .map(|n| {
        let mut n = n.parse::<f64>().unwrap();
        n = n.exp();
        n.to_string()
    })
    .collect::<Vec<String>>()
    .join(" ");

    (a, result)
    
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result: Vec<f64> = Vec::new();

    for val in b.iter() {
        let vall = val.clone();
        let mut f = 0.0;
        f += f64::from(vall);
       let res: f64 = f.ln();
        result.push(res);
    }
    (b, result)
}
