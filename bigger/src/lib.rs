use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    match h.values().max() {
        Some(max) => {
            *max
        }
        None => i32::MAX
    }
}