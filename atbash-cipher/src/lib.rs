use std::char;

pub fn encode(inp: &str) -> String {
    let inp: Vec<_> = inp.replace(" ", "")
        .to_lowercase()
        .split(|c| !char::is_alphanumeric(c))
        .collect::<Vec<&str>>()
        .concat()
        .bytes()
        // .chars()
        .collect();
    let mut result = String::new();
    let mut counter = 1;
    for c in inp {
        if c & 0b1000_0000 == 0b1000_0000 {
            continue; // weeding out all the multi-byte chars (aka non-ascii chars.)
        }
        if char::is_alphabetic((c as u8) as char) ||
           ((c as u16) >= b'a' as u16 && (c as u16) <= b'z' as u16) {
            result.push(((b'z' as u16 - ((c as u16) - b'a' as u16)) as u8) as char);
        } else if char::is_numeric((c as u8) as char) {
            result.push((c as u8) as char);
        }

        if counter % 5 == 0 {
            result.push_str(" ");
        }
        counter += 1;
    }
    result.trim_right().to_string()
}

pub fn decode(inp: &str) -> String {
    let inp = inp.replace(" ", "");
    inp.chars()
        .map(|c| if char::is_numeric(c) {
            c
        } else {
            (b'z' - (c as u8 - b'a')) as char
        })
        .collect()
}