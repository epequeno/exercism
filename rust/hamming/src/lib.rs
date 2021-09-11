pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut res: usize = 0;

    if s1.len() != s2.len() {
        return None;
    }

    let s2_chars = s2.chars().collect::<Vec<char>>();

    s1.chars().enumerate().for_each(|(ix, c)| {
        if c != s2_chars[ix] {
            res += 1;
        }
    });

    Some(res)
}
