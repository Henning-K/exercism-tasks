pub fn encode<S: Into<String>>(data: S) -> String {
    let data = data.into();

    let mut cur_char: Option<char> = None;
    let mut cur_char_count = 0;
    let mut buf = String::new();

    for c in data.chars() {
        match cur_char {
            Some(b) => {
                if b==c  {
                    cur_char_count += 1; // increment count of current char.
                } else {
                    if cur_char_count > 1 {
                        buf.push_str(&cur_char_count.to_string());
                    }
                    buf.push(b);
                    cur_char_count = 1; // reset count of consecutive chars.
                    cur_char = Some(c);
                }
            },
            None => {
                cur_char = Some(c);
                cur_char_count = 1;
            }
        }
    }

    match cur_char {
        None => {},
        Some(c) => {
            if cur_char_count > 1 {
                buf.push_str(&cur_char_count.to_string());
            }
            buf.push(c);
        }
    }

    buf
}

use std::char;
use std::iter;
use std::str::FromStr;

pub fn decode<S: Into<String>>(data: S) -> String {
    let mut data = data.into();
    let mut result = String::new();
    loop {
        if data.is_empty() {
            break;
        }
        let num_offset = data.find(|c| !char::is_numeric(c)).unwrap_or(0);
        let mut count = 1;
        if num_offset != 0 {
            let num: String = data.drain(..num_offset).collect();
            count = usize::from_str(&num).unwrap();
        }
        let c: char = data.drain(..1usize).next().unwrap();
        let buf: String = iter::repeat(c).take(count).collect();
        result.push_str(&buf);
    }

    result
}
