pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    // Check Rows
    for row in table {
        if row[0] == row[1] && row[1] == row[2] {
            return format!{"player {} won",row[0]};
        }
    }

    // Check Cols
    for col in 0..=2 {
       if table[0][col] == table[1][col] && table[1][col] == table[2][col] {
        return format!{"player {} won",table[0][col]};
       }
    }

    // check Diag
    if table[0][0] == table[1][1] && table[1][1] == table[2][2] {
        return format!{"player {} won",table[0][0]};
    }
    if table[0][2] == table[1][1] && table[1][1] == table[2][0] {
        return format!{"player {} won",table[0][2]};
    }

    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if player == table[0][0] && table[0][0] == table[1][1] && table[1][1] == table[2][2] {
        return true;
    }
    if player == table[0][2] && table[0][2] == table[1][1] && table[1][1] == table[2][0] {
        return true;
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        if player == row[0] && row[0] == row[1] && row[1] == row[2] {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..=2 {
        if player == table[0][col] && table[0][col] == table[1][col] && table[1][col] == table[2][col] {
         return true;
        }
    }
    false
}
