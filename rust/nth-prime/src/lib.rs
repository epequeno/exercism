// https://en.wikipedia.org/wiki/Primality_test#Pseudocode
fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let n = (n + 1) as usize;

    let mut ans = vec![];
    let mut i = 0;
    loop {
        if is_prime(i) {
            ans.push(i);
        }
        i += 1;

        if ans.len() == n {
            return *ans.last().unwrap();
        }
    }
}
