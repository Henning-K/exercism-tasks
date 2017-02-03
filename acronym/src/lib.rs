use std::borrow::Cow;

fn sanitize<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();

    if input.contains(|c: char| !c.is_alphabetic()) {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            if !c.is_alphabetic() {
                output.push(' ');
            } else {
                output.push(c);
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}

#[allow(unused_variables)]
pub fn abbreviate<'a, S: Into<&'a str>>(inp: S) -> Cow<'a, str> {
    let input = inp.into();
    let input = sanitize(input);
    let mut res = String::new();
    for s in input.split_whitespace() {
        res.push(s.chars().nth(0).unwrap());
        if s.chars().all(|c| c.is_uppercase()) {
            continue;
        }
        for c in s.chars().skip(1).filter(|c| c.is_uppercase()) {
            res.push(c);
        }
    }
    Cow::Owned(res.to_uppercase())
}
