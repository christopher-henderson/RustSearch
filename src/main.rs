extern crate nqueens;
extern crate time;

use nqueens::Backtracker;
use std::io;

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
	let start = time::PreciseTime::now();
	let found = nqueens::Queen::backtrack(nqueens::Queen::new(0, 0, n));
    let end = time::PreciseTime::now();
    println!("found {} solutions in {} seconds", found, start.to(end));
}
