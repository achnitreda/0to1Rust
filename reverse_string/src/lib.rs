pub fn rev_str(input: &str) -> String {
    let mut string : String = String::new();
    for c in input.chars().rev(){
        string.push(c);
    }
    string
}