use std::io::{self, Read};
use std::iter::Cycle;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    let (mut double_count, mut triple_count) = (0, 0);
	for line in buffer.lines() {
		let mut map = HashMap::new();
		for letter in line.chars() {
			let mut count = map.entry(letter).or_insert(0);
			*count += 1;
		}
		let double: bool = map.iter().filter(|(key, value)| **value == 2).count() > 0;
		let triple: bool = map.iter().filter(|(key, value)| **value == 3).count() > 0;
		if double {
			double_count += 1;
		}
		if triple {
			triple_count += 1;
		}

	}

	println!("{:?}", (double_count, triple_count, double_count * triple_count));

    Ok(())
}

struct ChronalCalibration {
	change_sequence: Cycle<std::vec::IntoIter<i32>>,
	period: usize,
	frequency: i32,
}

impl ChronalCalibration {
    fn new(input: &str) -> ChronalCalibration {
		let mut change_sequence = Vec::new();
		for line in input.lines() {
			let (sign, rest) = line.split_at(1);
	    	let adjustment: i32 = rest.parse().unwrap();
	    	match sign {
	    		"-" => change_sequence.push((0 - adjustment)),
	    		"+" => change_sequence.push(adjustment),
	    		_ => panic!("unexpected {:?}", sign)
	    	}
		}
		let period = change_sequence.len();
		let cycle = change_sequence.into_iter().cycle();
		ChronalCalibration{ change_sequence: cycle, period: period, frequency: 0}
    }
}

impl Iterator for ChronalCalibration {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.frequency += self.change_sequence.next().unwrap();
        Some(self.frequency)
    }
}

fn problem_1_1(input: &str) -> i32 {
	let mut frequencies = ChronalCalibration::new(input);
	let period = frequencies.period;
	frequencies.nth(period - 1).unwrap()
}

fn problem_1_2(input: &str) -> i32 {
	let mut frequencies = ChronalCalibration::new(input);
	let mut set: HashSet<_> = HashSet::new(); // dedup
	
	loop {
		let item = frequencies.next().unwrap();
		if !set.insert(item) {
			return item;
		}
	}
}
