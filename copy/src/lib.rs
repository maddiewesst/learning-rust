pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let mut sum = 0.0;
    sum += f64::from(c);
    (c, sum.exp(), sum.ln())
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
        // let parts  = val.split(' ').collect::<Vec<&f64>>();
        let mut f = 0.0;
        f += f64::from(vall);
       let res: f64 = f.ln();
        result.push(res);
    }
    (b, result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: i32 = 0;
        let b = String::from("1 2 4 5 6");
        let c = vec![1, 2, 4, 5];

        let result = vec_function(c);
        assert_eq!(result, ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
}
