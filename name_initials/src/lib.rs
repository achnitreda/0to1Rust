pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut ans : Vec<String> = vec![];
    for str in names{
        let mut val : String = String::new();
        for s in str.split_whitespace() {
            if val.ends_with(".") {
                val.push(' ');
            } 
            val += &s[..1];
            val.push('.');
        }
        ans.push(val)
    }
    ans
}