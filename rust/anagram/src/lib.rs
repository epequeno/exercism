use std::collections::HashSet;

fn make_fingerprint(word: &str) -> Vec<char> {
    let mut fp: Vec<char> = word.to_lowercase().chars().collect();
    fp.sort_unstable();
    fp
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    let fp = make_fingerprint(word);
    possible_anagrams.iter().for_each(|a| {
        let test_fp = make_fingerprint(a);

        if fp == test_fp && word != *a {
            res.insert(*a);
        }
    });
    res
}
