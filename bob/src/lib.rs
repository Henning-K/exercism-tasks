pub fn reply(input: &str) -> String {
    // Nonsensical addressing of Bob.
    if input.len() == 0 {
        return "Fine. Be that way!".to_string();
    }

    // Question.
    match input.chars().last() {
        Some('?') => return "Sure.".to_string(),
        _ => {}
    }

    // Yelling.
    if input.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        return "Whoa, chill out!".to_string();
    }

    "Whatever.".to_string()
}
