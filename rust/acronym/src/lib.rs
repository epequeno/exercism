pub fn abbreviate(phrase: &str) -> String {
    let mut res: Vec<char> = Vec::new();
    let phrase = phrase.replace("-", " ");
    phrase.split_whitespace().for_each(|word| {
        let mut test_chars = word.chars().filter(|c| c.is_ascii_alphabetic());
 
        if word == word.to_uppercase() {
            res.push(test_chars.next().unwrap_or_default())
        } else if word == word.to_lowercase() {
            res.push(test_chars.next().unwrap_or_default().to_ascii_uppercase())
        } else {
            test_chars.for_each(|c| {
                if c.is_uppercase() {
                    res.push(c);
                }
            });
        }
    });

    res.into_iter().collect()
}
