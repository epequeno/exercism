pub fn build_proverb(list: &[&str]) -> String {
    let mut ans = String::new();
    if list.is_empty() {
        return ans;
    }
    for i in 0..(list.len() - 1) {
        let first = list[i];
        let second = list[i + 1];
        ans += &format!("For want of a {} the {} was lost.\n", first, second);
    }
    ans += &format!("And all for the want of a {}.", list[0]);
    ans
}
