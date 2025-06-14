pub fn to_url(s: &str) -> String {
    let string : String = s.to_string();

    string.replace(" ","%20")
}