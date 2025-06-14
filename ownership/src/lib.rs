pub fn first_subword(mut s: String) -> String {
    for (i,c) in s.chars().enumerate() {
        if c == '_' || i != 0 && c >= 'A' && c <= 'Z' {
            s = (&s[..i]).to_string();
            break
        }
    }
    s
}