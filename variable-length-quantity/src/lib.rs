/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();

    for number_ in values.iter() {
        let mut number = *number_;
        if number == 0 {
            result.push(0u8);
            continue;
        }
        let mut temp_res = Vec::with_capacity(4);
        while number > 0 {
            let mut temp = (number & 0x7f) as u8;
            number >>= 7;
            if !temp_res.is_empty() {
                temp |= 0x80;
            }
            temp_res.push(temp);
        }
        temp_res.reverse();
        result.append(&mut temp_res);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut res = vec![];

    let mut bytes = bytes.iter().map(|&i| i).peekable();
    let mut value = 0;
    while let Some(c) = bytes.next() {
        if (value & 0xfe_00_00_00) > 0 {
            return Err("Overflow would occur.");
        }

        value = (value << 7) | (c & 0x7f) as u32;

        if c & 0x80 == 0 {
            res.push(value);
            value = 0;
        } else {
            match bytes.peek() {
                None => {
                    return Err("Incomplete byte sequence.");
                }
                _ => {}
            }
        }

    }

    Ok(res)
}
