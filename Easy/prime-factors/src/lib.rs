pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut result = n;
    for i in 2..=n {
        while result % i == 0 {
            factors.push(i);
            result = result / i;
        }
    }
    factors
}
