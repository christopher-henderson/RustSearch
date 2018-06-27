extern crate nqueens;
extern crate time;

use std::io;

#[derive(Debug, Clone)]
pub struct Queen {
    pub column: i32,
    pub row: i32,
    pub n: i32,
    current: i32
}

impl Queen {
	pub fn new(column: i32, row: i32, n: i32) -> Queen {
		Queen{column, row, n, current: 0}
	}
}

impl Iterator for Queen {
	type Item = Queen;

	fn next(&mut self) -> Option<Queen> {
		self.current += 1;
		if self.current > self.n {
			return None;
		}
		Some(Queen::new(self.column + 1, self.current, self.n))
	}
}

fn n_from_input() -> i32 {
	loop {
		let mut n_string = String::new();
		println!("Enter target N value:");
		io::stdin().read_line(&mut n_string)
			.expect("Failed to read line");
		match n_string.trim().parse() {
			Ok(num) => return num,
			Err(err) => {println!("{}", err); continue;}
		}
	}
}

fn main() {
	let n = n_from_input();
	let mut answers: std::vec::Vec<std::vec::Vec<Queen>> = vec![];
	let mut found = 0;
	let fcg = Queen::new(0, 0, n);
	let start = time::PreciseTime::now();
	////////////////////
	nqueens::search(fcg, 
		// Reject
		&mut |solution: &[Queen], candidate: &Queen| {
			let column = candidate.column;
			let row = candidate.row;
			for queen in solution.iter() {
				let r = queen.row;
				let c = queen.column	;
				if (row == r) || (column == c) || (row + column == r + c) || (row - column == r - c) {
					return true;
				}
			}
			false
		},
		// Accept
		&mut |solution: &[Queen]| {
			if solution.len() > 0 && solution.len() == unsafe{solution.get_unchecked(0)}.n as usize {
				answers.push(solution.iter().map(|q| q.clone()).collect());
				found += 1;
				return true;
			}
			false
		}
	);
	////////////////////
    let end = time::PreciseTime::now();
    println!("found {} solutions in {} seconds", found, start.to(end));
}
