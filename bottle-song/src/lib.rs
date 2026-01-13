pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut recite_string: String = String::new();
    let mut bottles = start_bottles;
    let mut remaining = take_down;
    fn integer_to_en_us(n: u32) -> &'static str {
        match n {
            0 => "no",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => "many",
        }
    }

    while remaining > 0 {
        let next_bottles = if bottles > 1 { bottles - 1 } else { 0 };
        let prv_bottles_plural = if bottles > 1 { "bottles" } else { "bottle" };
        let aftr_bottles_plural: &str = if next_bottles == 1 {
            "bottle"
        } else {
            "bottles"
        };
        recite_string.push_str(&format!(
            "{0} green {prv_bottles_plural} hanging on the wall,\n{0} green {prv_bottles_plural} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {1} green {aftr_bottles_plural} hanging on the wall.\n\n",
    integer_to_en_us(bottles).chars().next().unwrap().to_uppercase().collect::<String>() + &integer_to_en_us(bottles)[1..],
            integer_to_en_us(next_bottles)
        ));
        bottles -= 1;
        remaining -= 1;
    }

    recite_string.trim_end().to_string()
}
