use std::collections::HashMap;

pub fn count<'a, S: Into<&'a str>>(nuc: char, sequence: S) -> usize {
    let input = sequence.into();
    input.chars().filter(|c| *c == nuc).count()
}

pub fn nucleotide_counts<'a, S: Into<&'a str>>(seq: S) -> HashMap<char, usize> {
    let input = seq.into();
    let mut res = HashMap::new();
    for c in "ATCG".chars() {
        res.insert(c, count(c, input));
    }
    res
}
