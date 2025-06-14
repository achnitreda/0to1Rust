pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = (c as f64).abs().ln();

    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let og = a;

    let mut res = String::new();
    
    for s in og.split_whitespace() {
        let mut n : f64 = s.parse().expect("");
        n = n.exp();
        res += &n.to_string();
        res.push(' ');
    }

    (og, res[..res.len()-1].to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let x = b;
    let mut res = vec![];
    for c in x.clone() {
        let ln = (c as f64).abs().ln();
        res.push(ln);
    }
    (x, res)
}