use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
   let mut set = HashSet::new();

   for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            set.insert(c);
        }
   }

   if set.len() == 26 {
    return true
   }

   false
}