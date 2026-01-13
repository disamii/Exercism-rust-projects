pub fn square(s: u32) -> u64 {
    if s== 0 || s > 64 {
        panic!("not valid square")
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
