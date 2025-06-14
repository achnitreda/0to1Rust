pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {  
    for c in v.as_bytes() {
        if 32 < *c && *c > 127 {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let mut i : usize = 0;
    loop {
        if  i+pat.len() < v.len() && v[i..i+pat.len()] == *pat {
            return true;
        }else if i+pat.len() >= v.len() {
            break
        }
        i +=1;
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index],&v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    let mut i : usize = 0;
    for c in v.chars() {
        if c == pat {
            return i
        }
        i += 1;
    }
    return usize::MAX
}