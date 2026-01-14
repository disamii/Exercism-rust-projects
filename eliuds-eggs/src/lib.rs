pub fn egg_count(display_value: u32) -> usize {
    let mut n = display_value; // temporary mutable copy
    let mut count = 0;

    while n > 0 {
        if n & 1 == 1 {
            count += 1;
        }
        n >>= 1; // shift the copy, not the original
    }

    count
}
