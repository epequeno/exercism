use std::collections::HashMap;

pub fn sing(start: isize, end: isize) -> String {
	let mut lines = Vec::new();
	for line in (end..start+1).rev() {
		lines.push(verse(line));
	}
	let res = lines.join("\n");
	res

}

fn pluralize(n: isize) -> HashMap<isize, String> {
	let mut res = HashMap::new();
	if n >= 2 {
		res.insert(n, String::from("bottles"));
	} else if n == 1 {
		res.insert(n, String::from("bottle"));
	} else {
		res.insert(n, String::from("no more bottles"));
	}
	res
}

pub fn verse(num_bottles: isize) -> String {
	let one_less = num_bottles - 1;
	let first_part = pluralize(num_bottles);
	let second_part = pluralize(one_less);
	if one_less == 0 {
		format!("{} {} of beer on the wall, {} {} of beer.\nTake it down and pass it around, {} of beer on the wall.\n", num_bottles, first_part[&num_bottles], num_bottles, first_part[&num_bottles], second_part[&(num_bottles-1)])
	} else if num_bottles == 0 {
		String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
	} else {
		format!("{} {} of beer on the wall, {} {} of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", num_bottles, first_part[&num_bottles], num_bottles, first_part[&num_bottles], num_bottles-1, second_part[&(num_bottles-1)])
	}

}
