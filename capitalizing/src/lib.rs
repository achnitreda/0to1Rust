pub fn capitalize_first(input: &str) -> String {
    if input.len() == 0 {
        return String::new()
    }
    let f = input.chars().nth(0).expect("").to_ascii_uppercase();
    f.to_string()+&input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut ans = String::new();

    for (i,c) in input.chars().enumerate() {
        if i == 0 || i>0 && input.chars().nth(i-1).expect("") == ' ' || input.chars().nth(i-1).expect("") == '\t' {
            let x = c.to_ascii_uppercase();
            ans.push(x);
        }else{
            ans.push(c);
        }
    }

    ans
}

pub fn change_case(input: &str) -> String {
    let mut ans  = String::new();
    for c in input.chars() {
        if c >= 'a' && c <= 'z' {
            let f = c.to_ascii_uppercase();
            ans.push(f)
        } else if c >= 'A' && c <= 'Z' {
            let f = c.to_ascii_lowercase();
            ans.push(f)
        }else {
            ans.push(c)
        }
    }
    ans
}