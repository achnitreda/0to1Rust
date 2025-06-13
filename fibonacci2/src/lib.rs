pub fn fibonacci(n: u32) -> u32 {
    let i : usize = n as usize;
    let mut nums : Vec<u32> = vec![0,1,1,2];
    if i < nums.len() {
        return nums[i];
    }
    loop {
        if i == nums.len()-1{
            break
        }
        let n : u32 = nums[nums.len()-1]+nums[nums.len()-2];
        nums.push(n)
    }
    return nums[i]
}