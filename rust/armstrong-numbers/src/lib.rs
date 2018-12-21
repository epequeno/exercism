pub fn is_armstrong_number(num: u32) -> bool {
    let s = format!("{}", num);
    let power = s.len() as u32;
    let mut total = 0;
    for c in s.chars() {
        let n: u32 = c.to_string().parse().unwrap();
        total += n.pow(power);
    }
    num == total
}
