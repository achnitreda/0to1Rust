pub fn pig_latin(text: &str) -> String {
    // aeiou
    // igloo =>  igloo + ay
    // square =>  are + squ + ay
    // queen => ueen qay

    println!("---> {text}");

    let mut ans = String::new();

    for (i,c) in text.chars().enumerate() {
        if ['a','e','i','o','u'].contains(&c) {
            if i == 0 {
                ans += &text[i..];
            } else if !text.starts_with("q") && c == 'u' && text.chars().nth(i-1) == Some('q') {
                ans += &(text[i+1..].to_string()+&text[..i+1]);
            } else {
                ans += &(text[i..].to_string()+&text[..i]);
            } 
            break
        }
    }

    ans+"ay"
}