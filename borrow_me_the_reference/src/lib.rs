pub fn delete_and_backspace(s: &mut String) {
    // remove (-)
    let chars : Vec<char> = s.chars().collect();
    let mut  res = String::new();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '-'{
            res.pop();
            i+=1;
        }else {
            res.push(chars[i]);
            i+=1;
        }
    }

    // "bo+er+++sskro++lcw"
    let rev_str : Vec<char> = res.chars().rev().collect();
    let mut ans = String::new();

    let mut j = 0;
    while j < rev_str.len() {
        if rev_str[j] == '+'{
            ans.pop();
            j+=1;
        }else {
            ans.push(rev_str[j]);
            j+=1;
        }
    }

    let x : String = ans.chars().rev().collect();

    *s = x
}

pub fn do_operations(v: &mut [String]) {
    // "10-3"
    let mut ans : Vec<String> = vec![];

    for s in v.iter() {
        if s.contains("-") {
            let parts : Vec<&str> = s.split("-").collect();
            let n1 : i32 = parts[0].parse().unwrap();
            let n2 : i32 = parts[1].parse().unwrap();
            let res = n1 - n2;
            ans.push(res.to_string());
        }
        if s.contains("+") {
            let parts : Vec<&str> = s.split("+").collect();
            let n1 : i32 = parts[0].parse().unwrap();
            let n2 : i32 = parts[1].parse().unwrap();
            let res = n1 + n2;
            ans.push(res.to_string());
        }
    }

    for i in 0..ans.len() {
        v[i] = ans[i].clone();
    }
}
