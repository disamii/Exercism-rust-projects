const UNITS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut parts = Vec::new();
    let mut scale_index = 0;

    while n > 0 {
        let chunk = n % 1000;

        if chunk > 0 {
            let mut segment = encode_chunk(chunk);
            let scale = SCALES[scale_index];

            if !scale.is_empty() {
                segment.push(' ');
                segment.push_str(scale);
            }

            parts.push(segment);
        }

        n /= 1000;
        scale_index += 1;
    }

    parts.reverse();
    parts.join(" ")
}

fn encode_chunk(n: u64) -> String {
    let mut parts = Vec::new();

    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
        parts.push(format!("{} hundred", UNITS[hundreds as usize]));
    }

    if remainder > 0 {
        if remainder < 20 {
            parts.push(UNITS[remainder as usize].to_string());
        } else {
            let tens = remainder / 10;
            let ones = remainder % 10;

            if ones == 0 {
                parts.push(TENS[tens as usize].to_string());
            } else {
                parts.push(format!("{}-{}", TENS[tens as usize], UNITS[ones as usize]));
            }
        }
    }

    parts.join(" ")
}
