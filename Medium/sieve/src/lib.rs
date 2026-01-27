pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut cntr = Vec::new();
    if upper_bound < 2 {
        return cntr;
    }

    for n in 2..=upper_bound {
        let mut is_prime = true;
        for divider in 2..n {
            if n % divider == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            cntr.push(n);
        }
    }

    cntr
}
