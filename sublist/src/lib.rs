#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Superlist,
    Sublist,
}

#[allow(unused_variables)]
pub fn sublist<T: PartialEq>(v1: &[T], v2: &[T]) -> Comparison {
    if v1.len() > v2.len() {
        if v2.is_empty() {
            return Comparison::Superlist;
        }
        let l1 = v1.len();
        let l2 = v2.len();

        for (i, e) in v1.iter().enumerate() {
            if l1 - i < l2 {
                return Comparison::Unequal;
            }
            if *e == v2[0] {
                if v1.iter().skip(i).zip(v2.iter()).all(|(a, b)| *a == *b) {
                    return Comparison::Superlist;
                }
            }
        }
    }

    if v1.len() < v2.len() {
        if v1.is_empty() {
            return Comparison::Sublist;
        }
        let l1 = v1.len();
        let l2 = v2.len();

        for (i, e) in v2.iter().enumerate() {
            if l2 - i < l1 {
                return Comparison::Unequal;
            }
            if *e == v1[0] {
                if v2.iter().skip(i).zip(v1.iter()).all(|(a, b)| *a == *b) {
                    return Comparison::Sublist;
                }
            }
        }
    }

    if v1.iter().zip(v2.iter()).all(|(a, b)| *a == *b) {
        return Comparison::Equal;
    }

    Comparison::Unequal
}
