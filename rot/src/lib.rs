pub fn rotate(input: &str, key: i8) -> String {
    let mut ans = String::new();   

    let v = mod_neg(key,26);

    for c in input.chars() {
        if c.is_ascii_uppercase() {
            let x = (c as u8 - b'A' + v as u8)%26 + b'A';
            ans.push(x as char);
        }else if c.is_ascii_lowercase() {
            let x = (c as u8 - b'a' + v as u8)%26 + b'a';
            println!("{x}");
            ans.push(x as char);
        }else {
            ans.push(c);
        }
    }

    ans
}

fn mod_neg(n: i8, m: i8) -> i8 {
    let r = n % m;
    if r < 0 { r + m } else { r }
}