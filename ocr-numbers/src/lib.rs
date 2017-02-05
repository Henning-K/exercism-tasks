pub fn convert(input: &str) -> Result<String, ()> {
    if input.lines().count() % 4 != 0 || input.lines().any(|l| l.len() % 3 != 0) {
        return Err(());
    }

    // Separate the input into chunks of lines and feed those to `handle_line`.
    // If there is more than one 'line' of input, prepend lines with ','
    // starting with the second 'line' of numbers.
    let mut result = String::new();
    let mut iter = input.lines();
    for i in 0..(input.lines().count() / 4) {
        let line = iter.by_ref().take(4);
        let line: String = line.map(|l| l.to_string() + "\n").collect();
        if i > 0 {
            result.push(',');
        }
        result.push_str(&handle_line(&line));
    }

    Ok(result)
}

/// This handles each 'line' of the input, aka 4 actual lines of text each by
/// splitting it into 4x3 char chunks, feeding those to `handle_char`,
/// concatenating the returned chars and then returning the whole 'line'.
fn handle_line(input: &str) -> String {
    let line_lines: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut result = String::new();
    for char_nr in 0..(line_lines[0].len() / 3) {
        let mut curr_char = String::new();
        for i in 0..4 {
            for j in 0..3 {
                curr_char.push(line_lines[i][char_nr * 3 + j]);
            }
        }
        result.push(handle_char(&curr_char));
    }
    result
}

/// handle each 'number' in the input, aka 4x3 char chunks of the input.
/// This returns '?' if the characters was not recognized. Otherwise it returns
/// the recognized number.
fn handle_char(input: &str) -> char {
    match input {
        " _ | ||_|   " => '0',
        "     |  |   " => '1',
        " _  _||_    " => '2',
        " _  _| _|   " => '3',
        "   |_|  |   " => '4',
        " _ |_  _|   " => '5',
        " _ |_ |_|   " => '6',
        " _   |  |   " => '7',
        " _ |_||_|   " => '8',
        " _ |_| _|   " => '9',
        _ => '?',
    }
}
