pub fn encode(source: &str) -> String {
    let mut count = 0;
    let mut current: Option<char> = None;
    let mut encoded = String::new();

    for c in source.chars() {
        match current {
            Some(prev) if prev == c => {
                count += 1;
            }
            Some(prev) => {
                if count > 1 {
                    encoded.push_str(&count.to_string());
                }
                encoded.push(prev);
                current = Some(c);
                count = 1;
            }
            None => {
                current = Some(c);
                count = 1;
            }
        }
    }
    if let Some(last) = current {
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        encoded.push(last);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count: usize = 0;

    for c in source.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + (c as usize - '0' as usize);
        } else {
            let repeat = if count == 0 { 1 } else { count };
            decoded.push_str(&c.to_string().repeat(repeat));
            count = 0;
        }
    }

    decoded
}
