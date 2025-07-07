pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut x = 1; 
    let mut res = vec![];
    if arr.len() <= 1 {
        return res
    }
    for i in &arr {
        x *= i
    }
    for v in arr {
        res.push(x/v)
    }
    res
}