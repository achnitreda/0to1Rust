pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res = vec![];

    for i in 1..=50 {
        let x = i * 2;
        res.push(x * x);
    }

    res
}
