pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut res = Box::new(vec![]);

    for v in s.split_whitespace(){
        match v.strip_suffix("k") {
            Some(v) => {
                let s = v.replace(".","");
                let n = s.parse::<u32>().unwrap();
                res.push(n*100);
            },
            None => {
                let n = v.parse::<u32>().unwrap();
                res.push(n);
            }
        }
    }

    res
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    let mut res = vec![];

    for v in a.iter() {
       res.push(*v);
    }

    res
}
