use std::sync::Mutex;
use csp::ArcBufferedChannel;
use std::vec::Vec;
use std::sync::{RwLock};
use std::sync::Arc;
use std::thread;

extern crate num_cpus;
extern crate csp;

pub type Item<T> = Arc<RwLock<T>>;
pub type Core<T> = Vec<Item<T>>;
pub type CoreSlice<T> = [Item<T>];

pub fn search<T: 'static, R, A>(fcg: T, reject: R, accept: A)
where
	T: Iterator<Item = T> + Send + Sync,
	R: Fn(&CoreSlice<T>, &T) -> bool + Send + Sync + 'static,
	A: Fn(&CoreSlice<T>) -> bool + Send + Sync + 'static
{
	let core: Core<T> = vec![Arc::new(RwLock::new(fcg))];
	let workers = num_cpus::get();
	let _available = workers;
	let mut handles = vec![];
	let work_channel = csp::BufferedChannel::new(workers);
	let wg = csp::BufferedChannel::new(workers);
	let reject = Arc::new(reject);
	let accept = Arc::new(accept);
	let lock = Arc::new(Mutex::new(0));
	for _ in 0..workers {
		let work_channel = work_channel.clone();
		let reject = reject.clone();
		let accept = accept.clone();
		let wg = wg.clone();
		let lock = lock.clone();
		handles.push(thread::spawn(move || {
			engine(work_channel, wg, lock, reject, accept);
		}));
	}
	work_channel.send(core);
	wg.send(1);
	for handle in handles {
		handle.join().unwrap();
	}
}

fn engine<T, R, A>(work_channel: ArcBufferedChannel<Core<T>>, wg: ArcBufferedChannel<i32>, lock: Arc<Mutex<i32>>, reject: Arc<R>, accept: Arc<A>) 
where
	T: Iterator<Item = T> + Send + Sync,
	R: Fn(&CoreSlice<T>, &T) -> bool + Send + Sync + 'static,
	A: Fn(&CoreSlice<T>) -> bool + Send + Sync + 'static
{
	loop {
		let mut core = work_channel.recv();
		let mut root_pointer: usize = core.len() - 1;
		loop {
			let cand = core[root_pointer].write().unwrap().next();
			match cand {
				Some(candidate) => {
					if reject(&core[1..], &candidate) {
			    		continue;
			    	}
			    	core.push(Arc::new(RwLock::new(candidate)));
			    	if accept(&core[1..]) {
			    		core.pop();
			    		continue;
			    	}
			    	{
			    		match wg.try_send(1) {
				    		Ok(_) => {
				    			work_channel.send(core.clone());
				    			core.pop();
				    			continue;
				    		}
				    		Err(_) => ()
			    		}	
			    	}
			    	root_pointer += 1;
				},
				None => {
					core.pop();
					if root_pointer == 0 {
						break;
					}
					root_pointer -= 1;
				}
			}
		}
		{
		    match wg.try_send(1) {
				Ok(_) => {
					wg.recv();
					continue;
				}
				Err(_) => return
			}
		}
	}
}

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