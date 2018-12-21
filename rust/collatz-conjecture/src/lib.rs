pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut x = n;
    let mut counter = 0;
    loop {
        if x == 1 {
            break;
        }

        match x % 2 {
            0 => {
                x /= 2;
                counter += 1;
            }
            1 => {
                x = (x * 3) + 1;
                counter += 1;
            }
            _ => (),
        }
    }
    Some(counter)
}
