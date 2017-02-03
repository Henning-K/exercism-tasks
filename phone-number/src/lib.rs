pub fn number(s: &str) -> Option<String> {
    let mut clean_numbers_vec = s.chars()
                                 .filter(|c| c.is_numeric())
                                 .collect::<Vec<char>>();
    match clean_numbers_vec.len() {
        10 => {
            Some(clean_numbers_vec.iter().fold(String::new(), |mut acc, &c| {
                acc.push(c);
                acc
            }))
        }
        11 => {
            if clean_numbers_vec[0] == '1' {
                clean_numbers_vec.remove(0);
                return Some(clean_numbers_vec.iter().fold(String::new(), |mut acc, &c| {
                    acc.push(c);
                    acc
                }));
            } else {
                return None;
            }
        }
        e if e < 10 => None,
        e if e > 11 => None,
        _ => None,
    }
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).and_then(|mut st| {
        st.truncate(3);
        Some(st)
    })
}

pub fn pretty_print(s: &str) -> String {
    let num = match number(s) {
        None => {
            return String::from("invalid");
        }
        Some(v) => v,
    };
    let num = num.chars().collect::<Vec<char>>();
    format!("({}{}{}) {}{}{}-{}{}{}{}",
            num[0],
            num[1],
            num[2],
            num[3],
            num[4],
            num[5],
            num[6],
            num[7],
            num[8],
            num[9])
}
