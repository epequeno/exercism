pub fn is_valid(code: &str) -> bool {
    let mut res = Vec::new();
    let code = code.replace(" ", "");

    if code.len() <= 1 {
        return false;
    }

    if !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // add a zero to the beginning of uneven length inputs.
    // add to the beginning as we start identifiying digits to double by starting from the right.
    let c = if code.len() % 2 == 1 {
        format!("0{}", code)
    } else {
        String::from(code)
    };

    c.chars()
        .collect::<Vec<char>>()
        .as_slice()
        .chunks(2)
        .for_each(|chunk| {
            let a = chunk[0].to_digit(10).unwrap_or_default();
            let b = chunk[1].to_digit(10).unwrap_or_default();
            res.push(b);
            let mut product = a * 2;
            
            if product > 9 {
                product -= 9;
            }

            res.push(product);
        });

    res.iter().sum::<u32>() % 10 == 0
}
