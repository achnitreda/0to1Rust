use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut mp1 : HashMap<char, i32> = HashMap::new();
    let mut mp2 : HashMap<char, i32> = HashMap::new();

    for c in s1.chars() {
        let count = mp1.entry(c).or_insert(0);
        *count+=1;
    }

    for c in s2.chars() {
        let count = mp2.entry(c).or_insert(0);
        *count+=1;
    }

    mp1 == mp2
}
