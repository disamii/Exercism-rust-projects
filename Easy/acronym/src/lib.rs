pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut prev: Option<char> = None;

    for c in phrase.chars() {
        if !c.is_alphabetic() {
            if c != '\'' {
                prev = None;
            }
            continue;
        }

        let is_start = prev.is_none();
        let is_camel = prev.map(|p| p.is_lowercase() && c.is_uppercase()).unwrap_or(false);

        if is_start || is_camel {
            result.extend(c.to_uppercase());
        }

        prev = Some(c);
    }

    result
}
