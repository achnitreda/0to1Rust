#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.clone()
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value.push_str(&nb_to_append.to_string());
        self.clone()
    }
    fn remove_punctuation_marks(&mut self) -> Self {
        let mut value = String::new(); 
        for c in self.value.chars() {
            if c == '.' || c == ',' || c == '?' || c == '!' {
                continue
            }
            value.push(c);
        }
        self.value = value;
        self.clone()
    }
}