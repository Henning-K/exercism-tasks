extern crate regex;

use regex::Regex;

use std::collections::HashMap;

pub struct CodonTable<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonTable<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
        let fasta = vec![("R", "[AG]"),
                         ("Y", "[CTU]"),
                         ("K", "[GTU]"),
                         ("M", "[AC]"),
                         ("S", "[CG]"),
                         ("W", "[ATU]"),
                         ("B", "[CGTU]"),
                         ("D", "[AGTU]"),
                         ("H", "[ACTU]"),
                         ("V", "[ACG]"),
                         ("N", "[ACGTU]")];
        let substitution: HashMap<&str, &str> = fasta.into_iter().collect();

        let mut expr = codon.to_string();

        for (a, b) in substitution {
            if codon.contains(a) {
                expr = expr.replace(a, b);
            }
        }
        let re = Regex::new(&expr).unwrap();

        if codon.len() != 3 {
            return Err("Codon length is wrong.");
        }
        self.pairs.iter()
            .find(|&&(code, _name)| re.is_match(code))
            .ok_or("Not found.") // Option<T> -> Result<T, E>
            .map(|&(_code, name)| name) // Result<T, E> -> Result<U, E>
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonTable<'a> {
    CodonTable { pairs: pairs }
}