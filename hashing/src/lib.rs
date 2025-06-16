use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let total: i32 = list.iter().sum();

    (total as f64)/(list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut n = list.to_vec();
    n.sort();

    let i : usize = n.len()/2;

    if list.len()%2 == 0 {
        (n[i - 1]+ n[i]) / 2 as i32
    }else{
        return n[i]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut mp = HashMap::new();
    for v in list {
        let c = mp.entry(v).or_insert(0);
        *c +=1;
    }

     let binding = mp.clone();  
     let max = binding.values().max();

    for (k,v) in mp {
        if max == Some(&v) {
            return *k
        }
    }
    return i32::MAX
}