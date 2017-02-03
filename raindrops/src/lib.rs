pub fn raindrops(drops: i32) -> String {
    let mut result = String::new();
    let facts = factors(drops);

    if facts.contains(&3) {
        result = format!("{}", "Pling");
    }
    if facts.contains(&5) {
        result = format!("{}{}", result, "Plang");
    }
    if facts.contains(&7) {
        result = format!("{}{}", result, "Plong");
    }
    if result.is_empty() {
        result = format!("{}", drops)
    }
    result
}

fn factors(number: i32) -> Vec<i32> {
    let mut n = number;
    let mut res = Vec::new();

    for i in 2..n + 1 {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
    }

    res
}
