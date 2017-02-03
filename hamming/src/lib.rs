pub fn hamming_distance(dna0: &str, dna1: &str) -> Result<u32, String> {
    if dna0.len() != dna1.len() {
        return Err("Lengths of DNA strands don't match up.".to_string());
    }

    Ok(dna0.chars().zip(dna1.chars()).fold(0u32, |sum, (i, j)| {
        sum +
        match i == j {
            true => 0,
            false => 1,
        }
    }))
}
