use std::collections::HashMap;

pub fn word_count<'a, S: Into<&'a str>>(inp: S) -> HashMap<String, u32> {
    let input = inp.into().to_lowercase();
    let mut res = HashMap::new();
    let temp = input.split_whitespace();
    let mut temp_ = Vec::new();
    for s in temp {
        let ch = s.chars().filter(|c| c.is_alphanumeric());
        let mut temp_str = String::new();
        for c in ch {
            temp_str.push(c);
        }
        if temp_str.is_empty() {
            continue;
        }
        temp_.push(temp_str);
    }
    for word in temp_ {
        match res.get(&word.to_string()) {
            None => {
                res.insert(word.to_string(), 1);
            }
            Some(&n) => {
                res.insert(word.to_string(), n + 1);
            }
        }
    }
    println!("{:?}", res);
    res
}
