use std::char;

pub fn rotate<S: Into<String>>(data: S, rot: usize) -> String {
    let data = data.into();
    let rot = (rot % 26) as u8;
    data.chars().map(|c| {
        if char::is_lowercase(c) {
            let (a, c) = ('a' as u8, c as u8);
            (a + ((c - a + rot)%26)) as char
        } else if char::is_uppercase(c) {
            let (a, c) = ('A' as u8, c as u8);
            (a + ((c - a + rot)%26)) as char
        } else {
            c
        }
    }).collect()
}