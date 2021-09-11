use std::collections::HashSet;

fn make_fingerprint(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    let fp = make_fingerprint(word);
    possible_anagrams.iter().for_each(|candidate| {
        let candidate_fp = make_fingerprint(&candidate);
        if fp == candidate_fp && word.to_lowercase() != candidate.to_lowercase() {
            res.insert(*candidate);
        }
    });
    res
}
