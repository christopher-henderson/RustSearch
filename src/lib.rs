pub fn search<T>(fcg: T, reject: &mut FnMut(&[T], &T) -> bool, accept: &mut FnMut(&[T]) -> bool) where T: Iterator<Item = T> {
	let mut root_pointer: usize = 0;
	let mut core = vec![fcg];
	loop {
	    if let Some(c) = unsafe{core.get_unchecked_mut(root_pointer)}.next() {
	    	let mut candidate = c as T;
	    	if reject(&core[1..], &candidate) {
	    		continue;
	    	}
	    	core.push(candidate);
	    	if accept(&core[1..]) {
	    		core.pop();
	    		continue;
	    	}
	    	root_pointer += 1;
	    } else {
			core.pop();
			if core.len() == 0 {
				break;
			}
			root_pointer -= 1;
	    }
	}
}



// impl PartialEq for Queen {
// 	fn eq(&self, other: &Queen) -> bool {
// 		return self.column == other.column && self.row == other.row;
// 	}
// }

// impl Clone for Queen {
// 	fn clone(&self) -> Queen {
// 		Queen{column: self.column, row: self.row, n: self.n, current: self.current}
// 	}
// }

// fn reject(solution: &[Queen], candidate: &Queen) -> bool {
	// let column = candidate.column;
	// let row = candidate.row;
	// for queen in solution.iter() {
	// 	let r = queen.row;
	// 	let c = queen.column	;
	// 	if (row == r) || (column == c) || (row + column == r + c) || (row - column == r - c) {
	// 		return true;
	// 	}
	// }
	// false
// }

// fn accept(solution: &[Queen]) -> bool {
// 	solution.len() > 0 && solution.len() == unsafe{solution.get_unchecked(0)}.n as usize
// }

// pub fn backtrack(fcg: Queen) -> u32 {
// 	let mut found = 0;
// 	let mut root_pointer: usize = 0;
// 	let mut core: vec::Vec<Queen> = vec![fcg];
// 	loop {
// 	    if let Some(candidate) = unsafe{core.get_unchecked_mut(root_pointer)}.next() {
// 	    	if reject(&core[1..], &candidate) {
// 	    		continue;
// 	    	}
// 	    	core.push(candidate);
// 	    	if accept(&core[1..]) {
// 	    		found += 1;
// 	    		core.pop();
// 	    		continue;
// 	    	}
// 	    	root_pointer += 1;
// 	    } else {
// 			core.pop();
// 			if core.len() == 0 {
// 				break;
// 			}
// 			root_pointer -= 1;
// 	    }
// 	}
// 	found
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn must_reject() {
// 		let n = 4;
// 		assert!(reject(&vec![Queen::new(1, 1, n)][..], &Queen::new(2, 2, n)));
// 		assert!(reject(&vec![Queen::new(1, 1, n), Queen::new(2, 4, n)][..], &Queen::new(3, 4, n)));
// 		assert!(reject(&vec![Queen::new(1, 1, n), Queen::new(2, 4, n), Queen::new(3, 4, n)][..], &Queen::new(4, 3, n)));
// 	}

// // 1-
// //  0  0  1  0 
// //  1  0  0  0 
// //  0  0  0  1 
// //  0  1  0  0 
// // 2-
// //  0  1  0  0 
// //  0  0  0  1 
// //  1  0  0  0 
// //  0  0  1  0 
// 	#[test]
// 	fn must_not_reject() {
// 		let n = 4;
// 		assert!(!reject(&vec![Queen::new(1, 2, n)][..], &Queen::new(2, 4, n)));
// 		assert!(!reject(&vec![Queen::new(1, 2, n), Queen::new(2, 4, n)][..], &Queen::new(3, 1, n)));
// 		assert!(!reject(&vec![Queen::new(1, 2, n), Queen::new(2, 4, n), Queen::new(3, 1, n)][..], &Queen::new(4, 3, n)));

// 		assert!(!reject(&vec![Queen::new(1, 3, n)][..], &Queen::new(2, 1, n)));
// 		assert!(!reject(&vec![Queen::new(1, 3, n), Queen::new(2, 1, n)][..], &Queen::new(3, 4, n)));
// 		assert!(!reject(&vec![Queen::new(1, 3, n), Queen::new(2, 1, n), Queen::new(3, 4, n)][..], &Queen::new(4, 2, n)));
// 	}

// 	#[test]
// 	fn must_accept() {
// 		let n = 4;
// 	    assert!(accept(&vec![Queen::new(1, 3, n), Queen::new(2, 1, n), Queen::new(3, 4, n), Queen::new(4, 2, n)][..]));
// 	}

// 	#[test]
// 	fn must_not_accept() {
// 		let n = 4;
// 		assert!(!accept(&vec![Queen::new(1, 3, n), Queen::new(2, 1, n), Queen::new(3, 4, n)][..]));
// 	}

// 	#[test]
// 	fn correct_children() {
// 		let n = 4;
// 	    let mut fcg = Queen::new(0, 0, n);
// 	    let expected = vec![Queen::new(1, 1, n), Queen::new(1, 2, n), Queen::new(1, 3, n), Queen::new(1, 4, n)];
// 	    let mut got: vec::Vec<Queen> = vec::Vec::new();
// 	    while let Some(queen) = fcg.next() {
// 	    	got.push(queen);
// 	    }
// 	    assert_eq!(expected, got);
// 	}
// }