pub fn score(inp: &str) -> u32 {
    inp.to_lowercase().chars().fold(0u32, |acc, c| acc + lookup(&c))
}

fn lookup(c: &char) -> u32 {
    if vec!['q', 'z'].contains(c) {
        10u32
    } else if vec!['j', 'x'].contains(c) {
        8u32
    } else if c == &'k' {
        5u32
    } else if vec!['f', 'h', 'v', 'w', 'y'].contains(c) {
        4u32
    } else if vec!['b', 'c', 'm', 'p'].contains(c) {
        3u32
    } else if vec!['d', 'g'].contains(c) {
        2u32
    } else if "aeioulnrst".chars().collect::<Vec<char>>().contains(c) {
        1u32
    } else {
        0u32
    }
}
