pub fn number_logic(num: u32) -> bool {  
    // number => 154 
    // to strind => len() => 3
    // ro  [1,5,3]

    let arr : Vec<char> = num.to_string().chars().collect();

    println!("---> {:?}",arr);

    let mut res = 0;
    let count = arr.len() as u32;

    for n in arr {
        let num = n.to_string().parse::<u32>().unwrap();
        res += num.pow(count);
    }

    if u32::from(res) == num {
        return true
    }

    false
}   