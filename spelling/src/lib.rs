pub fn spell(n: u64) -> String {
    if n == 1_000_000 {
        return "one million".to_string()
    }

    if n == 0 {
        return "zero".to_string()
    }

    let mut res = String::new(); 

    if n >= 1000 {
        let t = n/1000;
        res.push_str(&under_1000(t));
        res.push_str(" thousand");

        if n%1000 != 0 {
            res.push(' ');
            res.push_str(&under_1000(n%1000));
        }

        return res
    }

    under_1000(n)
}

fn under_1000(n : u64) -> String {
    let mut res = String::new();

    if n >= 100 {
        res.push_str(&units(n/100));
        res.push_str(" hundred");

        if n%100 != 0 {
            res.push(' ');
            res.push_str(&under_100(n%100));
        }

        return res
    }

    under_100(n)
}

fn under_100(n : u64) -> String {
    if n < 20 {
        return units(n);
    }

    let t = n/10;
    let u = n%10;

    if u == 0 {
        return tens(n/10)
    }

    format!("{}-{}",tens(t),&units(u))
}

fn units(n : u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}

fn tens(n : u64) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "fourty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string(),
    }
}
