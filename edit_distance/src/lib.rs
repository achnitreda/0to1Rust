pub fn edit_distance(source: &str, target: &str) -> usize {
    let s_chars: Vec<char> = source.chars().collect();
    let t_chars: Vec<char> = target.chars().collect();
    let s_len = s_chars.len();
    let t_len = t_chars.len();

    // println!("source -> {:?}, {s_len}",s_chars);
    // println!("target -> {:?}, {t_len}",t_chars);

    // source -> ['a', 'l', 'i', 'g', 'n', 'm', 'e', 'n', 't'], 9
    // target -> ['a', 's', 's', 'i', 'g', 'n', 'm', 'e', 'n', 't'], 10

    let mut dp = vec![vec![0; t_len+1]; s_len + 1];


    for i in 0..=s_len {
        dp[i][0] = i;
    }

    for j in 0..=t_len {
        dp[0][j] = j;
    }

    // println!("dp ---> {:?}",dp);

    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    for i in 1..=s_len {
        for j in 1..=t_len {
            let mut cost = 1;
            if s_chars[i - 1] == t_chars[j - 1] {
                cost = 0;
            }
            dp[i][j] = 
            usize::min( usize::min(
            dp[i - 1][j] + 1, dp[i][j - 1] + 1),
            dp[i - 1][j - 1] + cost);
        }
    }


    println!("dp ---> {:?}",dp);

    dp[s_len][t_len]
}
