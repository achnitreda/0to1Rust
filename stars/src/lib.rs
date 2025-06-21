pub fn stars(n: u32) -> String {
    "*".repeat(2_i32.pow(n).try_into().unwrap())
}   