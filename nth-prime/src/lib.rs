pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    for i in 2.. {
        if is_prime(i) {
            if count == n {
                return i;
            }
            count += 1;
        }
    }
    0
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
