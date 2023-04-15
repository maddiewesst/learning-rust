
pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_c: Vec<char> = source.chars().collect();
    let target_c: Vec<char> = target.chars().collect();
    let source_len = source_c.len();
    let target_len = target_c.len();

    let mut dist = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 0..=source_len {
        dist[i][0] = i;
    }
    for j in 0..=target_len {
        dist[0][j] = j;
    }

    for i in 1..=source_len {
        for j in 1..=target_len {
            let mut cost = 1;
            if source_c[i - 1] == target_c[j - 1] {
                cost = 0;
            }
            dist[i][j] = usize::min(
                usize::min(dist[i - 1][j] + 1, dist[i][j - 1] + 1),
                dist[i - 1][j - 1] + cost,
            );
        }
    }

    dist[source_len][target_len]
}
