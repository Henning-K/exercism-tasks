use std::collections::HashMap;

fn count_bytes(word: &str) -> HashMap<char, u32> {
    let mut word_letters: HashMap<char, u32> = HashMap::new();
    for b in word.chars() {
        let count = word_letters.entry(b).or_insert(0);
        *count += 1;
    }
    word_letters
}

pub fn anagrams_for<'a>(word: &str, inputs: &'a [&str]) -> Vec<&'a str> {
    let word = word.to_lowercase();
    let word_bytes = count_bytes(&word);
    inputs.iter()
        .filter(|inp| {
            let inp_lower = inp.to_lowercase();
            word != inp_lower && count_bytes(&inp_lower) == word_bytes
        })
        .cloned()
        .collect()
}
