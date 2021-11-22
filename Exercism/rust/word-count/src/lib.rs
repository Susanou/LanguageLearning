use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {

    println!("{}", words);

    let words = words
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && !(c == '\''), " ")
        .split_whitespace()
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        });

    println!("{:?}", words);
    words
}
