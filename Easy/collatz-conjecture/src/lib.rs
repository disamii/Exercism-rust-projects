pub fn collatz(n: u64) -> Option<u64> {
    let mut result=n;
    if n == 0 {
        return None;
    }

    let mut steps = 0;

    while result != 1 {
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = result * 3 + 1;
        }
        steps += 1;
    }

    Some(steps)
}
