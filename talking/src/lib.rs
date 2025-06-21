pub fn talking(text: &str) -> &str {
    if text.trim().len() == 0 {
        return "Just say something!"
    }

    let has_lowercase = text.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = text.chars().any(|c| c.is_ascii_uppercase());
     let is_all_uppercase = !has_lowercase && has_uppercase;

    if is_all_uppercase {
        if text.ends_with("?") {
            return "Quiet, I am thinking!";
        }else { 
            return "There is no need to yell, calm down!"
        }
    } else {
        if text.ends_with("?") {
            return "Sure."
        }else {
            return "Interesting"
        }
    }
}