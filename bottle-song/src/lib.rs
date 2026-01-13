pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut recite_string: String = String::new();
    let mut inner_start_bottles = start_bottles;
    let mut inner_takedown = take_down;
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

    while inner_takedown > 0 {
        let next_bottles = if inner_start_bottles > 1 {
            inner_start_bottles - 1
        } else {
            0
        };
        let prv_bottles_plural = if inner_start_bottles > 1 {
            "bottles"
        } else {
            "bottle"
        };
        let aftr_bottles_plural: &str = if next_bottles==1 {
            "bottle"
        } else {
            "bottles"
        };

        recite_string.push_str(&format!(
            "{} green {prv_bottles_plural} hanging on the wall,\n{} green {prv_bottles_plural} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {aftr_bottles_plural} hanging on the wall.\n\n",
    integer_to_en_us(inner_start_bottles).chars().next().unwrap().to_uppercase().collect::<String>() + &integer_to_en_us(inner_start_bottles)[1..],
    integer_to_en_us(inner_start_bottles).chars().next().unwrap().to_uppercase().collect::<String>() + &integer_to_en_us(inner_start_bottles)[1..],
            integer_to_en_us(next_bottles)
        ));
        inner_start_bottles -= 1;
        inner_takedown -= 1;
    }

    recite_string.trim_end().to_string()
}
