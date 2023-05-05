pub fn edit_distance(source: &str, target: &str) -> usize {
    let len_source = source.chars().count();
    let len_target = target.chars().count();
    let mut dt: Vec<Vec<usize>> = vec![vec![0 as usize; len_target + 1]; len_source + 1];
    for i in 1..(len_source+1){
        dt[i][0] = i;
    }
    for j in 1..(len_target+1){
        dt[0][j] = j;
    }
    let mut sub_cost: usize;
    for j in 1..=len_target {
        for i in 1..=len_source {
            if source.chars().nth(i-1).unwrap() == target.chars().nth(j-1).unwrap(){
                sub_cost = 0;
            } else {
                sub_cost = 1;
            }
            dt[i][j] = std::cmp::min(
                dt[i-1][j] +1,
                std::cmp::min(dt[i][j-1] + 1,
                dt[i-1][j-1] + sub_cost)
            );
        }
    }
    return dt[len_source][len_target];
}