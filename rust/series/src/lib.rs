pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ans = Vec::new();
    let chars: Vec<char> = digits.chars().collect();
    let mut i = 0;
    loop {
        let end = i + len;
        if end > digits.len() {
            break;
        } else {
            let s = &chars[i..end];
            let mut part = String::new();
            for c in s {
                part += &c.to_string();
            }
            ans.push(part);
        }
        i += 1;
    }
    ans
}
