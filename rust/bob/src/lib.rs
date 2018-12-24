pub fn reply(msg: &'static str) -> String {
    let mut res = String::new();
    if msg.ends_with("?") {
        res.push_str("Sure.")
    } else if msg == "" {
        res.push_str("Fine. Be that way!")
    } else if msg == msg.to_uppercase() {
        res.push_str("Whoa, chill out!")
    } else {
        res.push_str("Whatever.")
    }
    res
}
