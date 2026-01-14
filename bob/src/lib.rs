pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }
    if trimmed.ends_with('?')
        & trimmed
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
        & message.chars().any(|c| c.is_alphabetic())
    {
        return "Calm down, I know what I'm doing!";
    }
    if !trimmed.ends_with('?')
        & trimmed
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
        & message.chars().any(|c| c.is_alphabetic())
    {
        return "Whoa, chill out!";
    }
    if trimmed.ends_with('?') {
        return "Sure.";
    }
    "Whatever."
}
