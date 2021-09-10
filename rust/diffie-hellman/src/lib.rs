use rand::Rng;

// h/t: https://exercism.org/tracks/rust/exercises/diffie-hellman/solutions/cthachuk
// Returns (x ** y) mod m, avoiding overflow.
fn pow_mod(x: u64, y: u64, m: u64) -> u64 {
    (0..y).fold(1, |acc, _| (acc * x) % m)
}

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub, a, p)
}
