extern crate rayon;
use rayon::prelude::*;
use std::fmt::Debug;
pub fn search<T>(
    fcg: T,
    reject: &mut (Fn(&[T], &T) -> bool + Send + Sync),
    accept: &mut (Fn(&[T]) -> bool + Send + Sync),
) where
    T: Iterator<Item = T> + Send + Sync + Copy + Debug,
{
    let roots: Vec<T> = fcg.map(|root| root).collect();
    let roots = roots.as_slice();
    roots.into_par_iter().for_each(|root| {
        let mut root_pointer: usize = 0;
        let mut core: Vec<T> = vec![root.clone()];
        loop {
            if let Some(candidate) = unsafe { core.get_unchecked_mut(root_pointer) }.next() {
                if reject(&core, &candidate) {
                    continue;
                }
                core.push(candidate);
                if accept(&core) {
                    core.pop();

                    continue;
                }
                root_pointer += 1;
            } else {
                core.pop();
                if root_pointer == 0 {
                    break;
                }
                root_pointer -= 1;
            }
        }
    });

    // loop {
    //     if let Some(candidate) = unsafe { core.get_unchecked_mut(root_pointer) }.next() {
    //         if reject(&core[1..], &candidate) {
    //             continue;
    //         }
    //         core.push(candidate);
    //         if accept(&core[1..]) {
    //             core.pop();
    //             continue;
    //         }
    //         root_pointer += 1;
    //     } else {
    //         core.pop();
    //         if root_pointer == 0 {
    //             break;
    //         }
    //         root_pointer -= 1;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make an assertion on the exat contents of the solution, since
    // NQueens of 4 is only two solutions long.
    #[test]
    fn exact_nqueens_test() {
        let nqueens_4: [[Queen; 4]; 2] = [
            [
                Queen::new(1, 2, 4),
                Queen::new(2, 4, 4),
                Queen::new(3, 1, 4),
                Queen::new(4, 3, 4),
            ],
            [
                Queen::new(1, 3, 4),
                Queen::new(2, 1, 4),
                Queen::new(3, 4, 4),
                Queen::new(4, 2, 4),
            ],
        ];
        let n = 4;
        let mut answers: Vec<Vec<Queen>> = vec![];
        let fcg = Queen::new(0, 0, n);
        ////////////////////
        search(
            fcg,
            // Reject
            &mut |solution: &[Queen], candidate: &Queen| {
                let column = candidate.column;
                let row = candidate.row;
                for queen in solution.iter() {
                    let r = queen.row;
                    let c = queen.column;
                    if (row == r)
                        || (column == c)
                        || (row + column == r + c)
                        || (row - column == r - c)
                    {
                        return true;
                    }
                }
                false
            },
            // Accept
            &mut |solution: &[Queen]| {
                if solution.len() > 0
                    && solution.len() == unsafe { solution.get_unchecked(0) }.n as usize
                {
                    // Aggregate answers in captured vector.
                    // answers.push(solution.iter().map(|q| q.clone()).collect());
                    return true;
                }
                false
            },
        );
        assert_eq!(answers, nqueens_4);
    }

    // Merely assert that there are the correct number of solutions.
    // @TODO knuckle down and format the data set for 92 chess boards for n = 8.
    #[test]
    fn nqueens_larger_test() {
        let n = 8;
        let mut answers: Vec<Vec<Queen>> = vec![];
        let fcg = Queen::new(0, 0, n);
        ////////////////////
        search(
            fcg,
            // Reject
            &mut |solution: &[Queen], candidate: &Queen| {
                let column = candidate.column;
                let row = candidate.row;
                for queen in solution.iter() {
                    let r = queen.row;
                    let c = queen.column;
                    if (row == r)
                        || (column == c)
                        || (row + column == r + c)
                        || (row - column == r - c)
                    {
                        return true;
                    }
                }
                false
            },
            // Accept
            &mut |solution: &[Queen]| {
                if solution.len() > 0
                    && solution.len() == unsafe { solution.get_unchecked(0) }.n as usize
                {
                    // Aggregate answers in captured vector.
                    // answers.push(solution.iter().map(|q| q.clone()).collect());
                    return true;
                }
                false
            },
        );
        assert_eq!(answers.len(), 92);
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Queen {
        pub column: i32,
        pub row: i32,
        pub n: i32,
        current: i32,
    }

    impl Queen {
        pub fn new(column: i32, row: i32, n: i32) -> Queen {
            Queen {
                column,
                row,
                n,
                current: 0,
            }
        }
    }

    // Equality for a queen need not take into account n and current.
    impl PartialEq for Queen {
        fn eq(&self, other: &Queen) -> bool {
            self.column == other.column && self.row == other.row
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

    fn reject_queen(solution: &[Queen], candidate: &Queen) -> bool {
        let column = candidate.column;
        let row = candidate.row;
        for queen in solution.iter() {
            let r = queen.row;
            let c = queen.column;
            if (row == r) || (column == c) || (row + column == r + c) || (row - column == r - c) {
                return true;
            }
        }
        false
    }

    fn accept_queen(solution: &[Queen]) -> bool {
        if solution.len() > 0 && solution.len() == unsafe { solution.get_unchecked(0) }.n as usize {
            // Aggregate answers in captured vector.
            // answers.push(solution.iter().map(|q| q.clone()).collect());
            return true;
        }
        false
    }

    // Meta test to ensure that our NQueens test fixure is correct
    #[test]
    fn must_reject() {
        let n = 4;
        assert!(reject_queen(
            &vec![Queen::new(1, 1, n)][..],
            &Queen::new(2, 2, n)
        ));
        assert!(reject_queen(
            &vec![Queen::new(1, 1, n), Queen::new(2, 4, n)][..],
            &Queen::new(3, 4, n)
        ));
        assert!(reject_queen(
            &vec![
                Queen::new(1, 1, n),
                Queen::new(2, 4, n),
                Queen::new(3, 4, n),
            ][..],
            &Queen::new(4, 3, n)
        ));
    }

    // Meta test to ensure that our NQueens test fixure is correct
    // Data is the following chess boards.
    //
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
        let n = 4;
        assert!(!reject_queen(
            &vec![Queen::new(1, 2, n)][..],
            &Queen::new(2, 4, n)
        ));
        assert!(!reject_queen(
            &vec![Queen::new(1, 2, n), Queen::new(2, 4, n)][..],
            &Queen::new(3, 1, n)
        ));
        assert!(!reject_queen(
            &vec![
                Queen::new(1, 2, n),
                Queen::new(2, 4, n),
                Queen::new(3, 1, n),
            ][..],
            &Queen::new(4, 3, n)
        ));

        assert!(!reject_queen(
            &vec![Queen::new(1, 3, n)][..],
            &Queen::new(2, 1, n)
        ));
        assert!(!reject_queen(
            &vec![Queen::new(1, 3, n), Queen::new(2, 1, n)][..],
            &Queen::new(3, 4, n)
        ));
        assert!(!reject_queen(
            &vec![
                Queen::new(1, 3, n),
                Queen::new(2, 1, n),
                Queen::new(3, 4, n),
            ][..],
            &Queen::new(4, 2, n)
        ));
    }

    // Meta test to ensure that our NQueens test fixure is correct
    #[test]
    fn must_accept() {
        let n = 4;
        assert!(accept_queen(
            &vec![
                Queen::new(1, 3, n),
                Queen::new(2, 1, n),
                Queen::new(3, 4, n),
                Queen::new(4, 2, n),
            ][..]
        ));
    }

    // Meta test to ensure that our NQueens test fixure is correct
    #[test]
    fn must_not_accept() {
        let n = 4;
        assert!(!accept_queen(
            &vec![
                Queen::new(1, 3, n),
                Queen::new(2, 1, n),
                Queen::new(3, 4, n),
            ][..]
        ));
    }

    // Meta test to ensure that our NQueens test fixure is correct
    #[test]
    fn correct_children() {
        let n = 4;
        let mut fcg = Queen::new(0, 0, n);
        let expected = vec![
            Queen::new(1, 1, n),
            Queen::new(1, 2, n),
            Queen::new(1, 3, n),
            Queen::new(1, 4, n),
        ];
        let mut got: Vec<Queen> = Vec::new();
        while let Some(queen) = fcg.next() {
            got.push(queen);
        }
        assert_eq!(expected, got);
    }
}
