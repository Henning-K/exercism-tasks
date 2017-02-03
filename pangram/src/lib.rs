pub fn is_pangram(inp: &str) -> bool {
    let characters = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let input = inp.to_lowercase()
                   .chars()
                   .filter(|c| c.is_alphabetic())
                   .collect::<Vec<char>>();
    characters.iter().all(|c| input.contains(&c))
}
