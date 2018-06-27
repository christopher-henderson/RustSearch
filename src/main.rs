extern crate nqueens;
extern crate time;

use std::io;

#[inline(always)]
fn reject(solution: &[nqueens::Queen], candidate: &nqueens::Queen) -> bool {
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
}

#[inline(always)]
fn accept(solution: &[nqueens::Queen]) -> bool {
	solution.len() > 0 && solution.len() == unsafe{solution.get_unchecked(0)}.n as usize
}

fn main() {
	let n: i32;
	loop {
		let mut n_string = String::new();
		println!("Enter target N value:");
		io::stdin().read_line(&mut n_string)
			.expect("Failed to read line");
		match n_string.trim().parse() {
			Ok(num) => {n = num; break;}
			Err(err) => {println!("{}", err); continue;}
		}
	}
	let queen = nqueens::Queen::new(0, 0, n);
	let start = time::PreciseTime::now();
	let found = nqueens::backtrack(queen, 
		|solution, candidate| {
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
		|solution| {
			solution.len() > 0 && solution.len() == unsafe{solution.get_unchecked(0)}.n as usize
		}
	);
	// let found = nqueens::backtrack(queen, reject, accept);
    let end = time::PreciseTime::now();
    println!("found {} solutions in {} seconds", found, start.to(end));
}
