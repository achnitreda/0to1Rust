pub fn scytale_cipher(message: String, i: u32) -> String {
    // "scytale Code" size=6 -> "sec yCtoadle"

    let len = message.len() as usize;
    let cols = i as usize;
    let rows = (cols + len -1)/cols;

    let mut res = vec![vec![' '; cols]; rows];

    for (i,c) in message.chars().enumerate() {
        let row = i/cols;
        let col = i%cols;
        res[row][col] = c;
    }

    let mut ans = String::new();

    for col in 0..cols {
        for row in 0..rows {
            ans.push(res[row][col]);
        }
    }

    ans.trim().to_string()
}