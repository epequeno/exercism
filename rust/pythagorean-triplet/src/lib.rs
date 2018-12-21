use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut ans: HashSet<[u32; 3]> = HashSet::new();
    'outer: for a in 1..sum {
        for b in 1..sum {
            if (a + b) >= sum {
                continue 'outer;
            }
            let c = sum - (a + b);
            if ((a < b) && (b < c)) && (a + b + c) == sum {
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    ans.insert([a, b, c]);
                }
            }
        }
    }
    ans
}
