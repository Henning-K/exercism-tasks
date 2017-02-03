pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }
    let mut res = vec![2];
    for i in (2..n + 1).filter(|x| x % 2 != 0) {
        if res.iter().all(|j| i % j != 0) {
            res.push(i);
        }
    }
    res
}
