use std::collections::HashMap;

pub fn raindrops(n: usize) -> String {
    let ordered_targets = vec![3, 5, 7];
    let mut targets = HashMap::new();
    targets.insert(3, "Pling");
    targets.insert(5, "Plang");
    targets.insert(7, "Plong");

    let mut res = vec![];

    for target in ordered_targets {
        if n % target == 0 {
            res.push(targets[&target])
        }
    }

    if res.len() > 0 {
        res.join("").to_string()
    } else {
        n.to_string()
    }
}
