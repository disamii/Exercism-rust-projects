pub fn translate(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    input
        .split_whitespace()
        .map(|word| {
            // Rule 1: starts with vowel, or "xr"/"yt"
            if word.starts_with(|c: char| vowels.contains(&c)) 
                || word.starts_with("xr") 
                || word.starts_with("yt") 
            {
                return format!("{}ay", word);
            }

            // Rule 2, 3, 4: leading consonant clusters
            let chars: Vec<char> = word.chars().collect();
            let mut cluster_len = 0;

            while cluster_len < chars.len() {
                let c = chars[cluster_len];

                // stop if vowel
                if vowels.contains(&c) {
                    break;
                }

                // Rule 3: consonant cluster ending with 'qu'
                if c == 'q' && cluster_len + 1 < chars.len() && chars[cluster_len + 1] == 'u' {
                    cluster_len += 1; // include 'u' in cluster
                }

                // Rule 4: consonants before 'y' count as cluster
                if c == 'y' && cluster_len != 0 {
                    break;
                }

                cluster_len += 1;
            }

            let (prefix, rest) = word.split_at(cluster_len);
            format!("{}{}ay", rest, prefix)
        })
        .collect::<Vec<_>>()
        .join(" ")
}
