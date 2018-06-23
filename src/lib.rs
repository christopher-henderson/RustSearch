use std::vec;

#[derive(Debug)]
pub struct Queen {
    column: i32,
    row: i32,
    current: i32
}

impl Queen {
	pub fn new(column: i32, row: i32) -> Queen {
		Queen{column, row, current: 0}
	}

	fn children(&mut self, n: u32) -> Option<Queen> {
		self.current += 1;
		if self.current > (n as i32) {
			return None;
		}
		Some(Queen::new(self.column + 1, self.current))
	}
}

impl PartialEq for Queen {
	fn eq(&self, other: &Queen) -> bool {
		return self.column == other.column && self.row == other.row;
	}
}

impl Clone for Queen {
	fn clone(&self) -> Queen {
		Queen{column: self.column, row: self.row, current: self.current}
	}
}

fn reject(solution: &vec::Vec<Queen>, candidate: &Queen) -> bool {
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

fn accept(solution: &vec::Vec<Queen>, n: u32) -> bool {
	solution.len() == (n as usize)
}

pub fn backtrack(mut root: Queen, n: u32) -> u32 {
	let mut solution: vec::Vec<Queen> = vec::Vec::new();
	let mut stack: vec::Vec<Queen> = vec::Vec::new();
	let mut found = 0;
	loop {
		if let Some(candidate) = root.children(n) {
			if reject(&solution, &candidate) {
				continue;
			}
			solution.push(candidate.clone());
			if accept(&solution, n) {
				found += 1;
				solution.pop();
				continue;
			}
			stack.push(root.clone());
			root = candidate;
		} else {
			if stack.len() == 0 {
				break;
			}
			solution.pop();
			root = stack.pop().expect("bounds underflow");
		}
	}
	found
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn must_reject() {
		assert!(reject(&vec![Queen::new(1, 1)], &Queen::new(2, 2)));
		assert!(reject(&vec![Queen::new(1, 1), Queen::new(2, 4)], &Queen::new(3, 4)));
		assert!(reject(&vec![Queen::new(1, 1), Queen::new(2, 4), Queen::new(3, 4)], &Queen::new(4, 3)));
	}

// 1-
//  0  0  1  0 
//  1  0  0  0 
//  0  0  0  1 
//  0  1  0  0 
// 2-
//  0  1  0  0 
//  0  0  0  1 
//  1  0  0  0 
//  0  0  1  0 
	#[test]
	fn must_not_reject() {
		assert!(!reject(&vec![Queen::new(1, 2)], &Queen::new(2, 4)));
		assert!(!reject(&vec![Queen::new(1, 2), Queen::new(2, 4)], &Queen::new(3, 1)));
		assert!(!reject(&vec![Queen::new(1, 2), Queen::new(2, 4), Queen::new(3, 1)], &Queen::new(4, 3)));

		assert!(!reject(&vec![Queen::new(1, 3)], &Queen::new(2, 1)));
		assert!(!reject(&vec![Queen::new(1, 3), Queen::new(2, 1)], &Queen::new(3, 4)));
		assert!(!reject(&vec![Queen::new(1, 3), Queen::new(2, 1), Queen::new(3, 4)], &Queen::new(4, 2)));
	}

	#[test]
	fn must_accept() {
	    assert!(accept(&vec![Queen::new(1, 3), Queen::new(2, 1), Queen::new(3, 4), Queen::new(4, 2)], 4));
	}

	#[test]
	fn must_not_accept() {
		assert!(!accept(&vec![Queen::new(1, 3), Queen::new(2, 1), Queen::new(3, 4)], 4));
	}

	#[test]
	fn correct_children() {
	    let mut fcg = Queen::new(0, 0);
	    let n = 4;
	    let expected = vec![Queen::new(1, 1), Queen::new(1, 2), Queen::new(1, 3), Queen::new(1, 4)];
	    let mut got: vec::Vec<Queen> = vec::Vec::new();
	    while let Some(queen) = fcg.children(n) {
	    	got.push(queen);
	    }
	    assert_eq!(expected, got);
	}
}