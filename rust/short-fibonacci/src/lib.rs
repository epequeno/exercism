use std::iter::Iterator;

pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

// h/t https://edgarluque.com/blog/rust-iterator-fibonacci
struct Fibo {
    a: u8,
    b: u8,
}

impl Fibo {
    fn new() -> Self {
        Fibo { a: 1, b: 0 }
    }
}

impl Iterator for Fibo {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.b;
        self.b = self.a;
        self.a += r;
        Some(r)
    }
}

pub fn fibonacci() -> Vec<u8> {
    Fibo::new().take(5).collect()
}
