use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut mp : HashMap<&'a str, usize> = HashMap::new();
    for v in words {
        let count = mp.entry(v).or_insert(0);
        *count+=1;
    }

    mp
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
