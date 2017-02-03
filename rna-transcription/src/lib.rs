#[derive(Debug, Clone, PartialEq)]
pub struct RibonucleicAcid {
    sequence: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeoxyribonucleicAcid {
    sequence: String,
}

#[allow(dead_code)]
impl RibonucleicAcid {
    pub fn new<S: Into<String>>(input: S) -> Self {
        RibonucleicAcid { sequence: input.into() }
    }
}

#[allow(dead_code)]
impl DeoxyribonucleicAcid {
    pub fn new<S: Into<String>>(input: S) -> Self {
        DeoxyribonucleicAcid { sequence: input.into() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let mut res = String::new();
        for c in self.sequence.chars() {
            match c {
                'A' => res.push('U'),
                'T' => res.push('A'),
                'G' => res.push('C'),
                'C' => res.push('G'),
                _ => {}
            }
        }
        RibonucleicAcid::new(res)
    }
}
