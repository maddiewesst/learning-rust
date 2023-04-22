pub fn get_diamond(c: char) -> Vec<String> {
    let mut rows = vec![];
    let n = c as usize - 'A' as usize + 1;
    for i in 0..n {
        let mut row = vec![' '; 2 * n - 1];
        let ch = (i + 'A' as usize) as u8 as char;
        row[n - 1 - i] = ch;
        row[n - 1 + i] = ch;
        rows.push(row.iter().collect());
    }
    for i in (0..n - 1).rev() {
        let mut row = vec![' '; 2 * n - 1];
        let ch = (i + 'A' as usize) as u8 as char;
        row[n - 1 - i] = ch;
        row[n - 1 + i] = ch;
        rows.push(row.iter().collect());
    }
    rows
}
