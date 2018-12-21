pub fn factors(n: u64) -> Vec<u64> {
    let mut ans: Vec<u64> = Vec::new();
    let mut x = n;
    let mut last_prime = 2;
    for i in 1..=((n / 2) + 1) {
        if is_prime(i, last_prime) {
            last_prime = i;
            while x % i == 0 {
                ans.push(i);
                x = x / i;
            }
        }
    }
    ans
}

// https://en.wikipedia.org/wiki/Primality_test#Pseudocode
fn is_prime(n: u64, start: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = start;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
