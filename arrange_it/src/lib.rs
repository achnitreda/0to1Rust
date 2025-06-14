pub fn arrange_phrase(phrase: &str) -> String {
    let x : Vec<&str> = phrase.split_whitespace().collect();
    let mut res : Vec<&str> = vec![""; x.len()];
    for word in x  {
        for c in word.chars() {
            if c.is_numeric(){
                let n = c.to_digit(10).unwrap() as usize;
                res[n-1] = word;
            }
        }
    }

    let s = res.join(" ");

    let mut ans = String::new();

    for c in s.chars() {
        if !c.is_numeric() {
            ans.push(c);
        }
    }
    
    ans
}