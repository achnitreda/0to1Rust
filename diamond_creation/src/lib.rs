pub fn get_diamond(c: char) -> Vec<String> {

    let mut res = vec![];
    let size = c as u8 - b'A'+1;

    for i in 0..size {
        let current_char = (b'A' + i) as char;
        let spaces = size - i - 1;

        let mut ans = String::new(); 

        ans.push_str(&" ".repeat(spaces as usize));
        ans.push(current_char);

        if i > 0 {
            let inner_spaces = 2 * i - 1;
            ans.push_str(&" ".repeat(inner_spaces as usize));
            ans.push(current_char);
        }

        ans.push_str(&" ".repeat(spaces as usize));

        res.push(ans);
    }

     for i in (0..size-1).rev() {
        let current_char = (b'A' + i) as char;
        let spaces = size - i - 1;

        let mut ans = String::new(); 

        ans.push_str(&" ".repeat(spaces as usize));
        ans.push(current_char);

        if i > 0 {
            let inner_spaces = 2 * i - 1;
            ans.push_str(&" ".repeat(inner_spaces as usize));
            ans.push(current_char);
        }

        ans.push_str(&" ".repeat(spaces as usize));

        res.push(ans);
    }

    res
}