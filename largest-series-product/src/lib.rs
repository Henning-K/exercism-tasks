
pub fn lsp(inp: &str, window_len: usize) -> Result<u64, String> {
    if window_len > inp.len() {
        return Err("Window length longer than input.".to_string());
    }
    if inp.chars().any(|c| !char::is_numeric(c)) {
        return Err("Non numeric character in input.".to_string());
    }
    if inp == "" || window_len == 0 {
        return Ok(1);
    }
    let inp: Vec<_> = inp.chars().map(|c| c.to_digit(10).unwrap()).collect();
    Ok(inp.windows(window_len)
        .map(|tup| tup.iter().map(|&i| i as u64).product())
        .max()
        .unwrap())
}
