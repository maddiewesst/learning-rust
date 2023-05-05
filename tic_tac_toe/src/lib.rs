pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return String::from("player X won");
    }
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return String::from("player O won");
    }
    for row in &table {
        for cell in row {
            if cell == &"#" {
                return String::from("tie");
            }
        }
    }
    String::from("tie")
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut diag1 = true;
    let mut diag2 = true;
    let n = table.len();
    for i in 0..n {
        diag1 = diag1 && (table[i][i] == player);
        diag2 = diag2 && (table[i][n-i-1] == player);
    }
    diag1 || diag2
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let n = table.len();
    for i in 0..n {
        let mut win = true;
        for j in 0..n {
            win = win && (table[i][j] == player);
        }
        if win {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let n = table.len();
    for i in 0..n {
        let mut win = true;
        for j in 0..n {
            win = win && (table[j][i] == player);
        }
        if win {
            return true;
        }
    }
    false
}
