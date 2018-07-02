use std::sync::Mutex;
use csp::ArcBufferedChannel;
use std::vec::Vec;
use std::sync::{RwLock};
use std::sync::Arc;
use std::thread;

extern crate num_cpus;
extern crate csp;

pub type Core<T> = Vec<T>;
pub type CoreSlice<T> = [T];

pub fn search<T: 'static, R, A>(mut fcg: T, reject: R, accept: A)
where
	T: Iterator<Item = T> + Send + Sync + Clone,
	R: Fn(&CoreSlice<T>, &T) -> bool + Send + Sync + 'static,
	A: Fn(&CoreSlice<T>) -> bool + Send + Sync + 'static
{
	let workers = num_cpus::get();
	let mut handles = vec![];
	let reject = Arc::new(reject);
	let accept = Arc::new(accept);
	let supplicants = workers - 1;
	let work_channel: ArcBufferedChannel<Option<Core<T>>> = csp::BufferedChannel::new(supplicants);
	let wg: ArcBufferedChannel<i32> = csp::BufferedChannel::new(supplicants);
	let mut count = 0;


	let master_wg = wg.clone();
	let master_work_channel = work_channel.clone();
	let master_reject = reject.clone();
	let master_accept = accept.clone();
	handles.push(thread::spawn(move || {
		let mut root_pointer: usize = 0;
		let mut core = vec![fcg];
		loop {
		    if let Some(candidate) = unsafe{core.get_unchecked_mut(root_pointer)}.next() {
		    	if master_reject(&core[1..], &candidate) {
		    		continue;
		    	}
		    	core.push(candidate);
		    	if master_accept(&core[1..]) {
		    		core.pop();
		    		continue;
		    	}
		    	match master_wg.try_send(1) {
		    		Ok(_) => {
		    			let work = core[1..].to_vec().clone();
		    			master_work_channel.send(Some(work));
		    			core.pop();
		    			continue;;
		    		},
		    		Err(_) => ()
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
		for _ in 0..supplicants {
			master_work_channel.send(None);
		}
	}));

	for _ in 1..workers {
		let reject = reject.clone();
		let accept = accept.clone();
		let wg = wg.clone();
		let work_channel = work_channel.clone();
		handles.push(thread::spawn(move || {
			loop {
				let mut core;
				if let Some(work) = work_channel.recv() {
					core = work;
				} else {
					return;
				}
				let mut root_pointer: usize = core.len() - 1;
				loop {
				    if let Some(candidate) = unsafe{core.get_unchecked_mut(root_pointer)}.next() {
				    	if reject(&core[..], &candidate) {
				    		continue;
				    	}
				    	core.push(candidate);
				    	if accept(&core[..]) {
				    		core.pop();
				    		continue;
				    	}
				    	root_pointer += 1;
				    } else {
				    	if core.len() == 0 {
				    		break;
				    	}
						core.pop();
						root_pointer -= 1;
				    }
				}
				wg.recv();
			}
		}));
	}
	for handle in handles {
		handle.join().unwrap();
	}	
}

// fn engine<T, R, A>(fcg: T, core:  reject: Arc<R>, accept: Arc<A>) 
// where
// 	T: Iterator<Item = T> + Send + Sync,
// 	R: Fn(&CoreSlice<T>, &T) -> bool + Send + Sync + 'static,
// 	A: Fn(&CoreSlice<T>) -> bool + Send + Sync + 'static
// {

// }
// fn engine<T, R, A>(work_channel: ArcBufferedChannel<Option<Core<T>>>, wg: ArcBufferedChannel<i32>, lock : ArcBufferedChannel<i32>, reject: Arc<R>, accept: Arc<A>) 
// where
// 	T: Iterator<Item = T> + Send + Sync,
// 	R: Fn(&CoreSlice<T>, &T) -> bool + Send + Sync + 'static,
// 	A: Fn(&CoreSlice<T>) -> bool + Send + Sync + 'static
// {
// 	loop {
// 		let mut core;
// 		if let Some(work) = work_channel.recv() {
// 			core = work;
// 		} else {
// 			return;
// 		}
// 		let mut root_pointer: usize = core.len() - 1;
// 		loop {
// 			let cand = core[root_pointer].write().unwrap().next();
// 			match cand {
// 				Some(candidate) => {
// 					if reject(&core[1..], &candidate) {
// 			    		continue;
// 			    	}
// 			    	core.push(Arc::new(RwLock::new(candidate)));
// 			    	if accept(&core[1..]) {
// 			    		core.pop();
// 			    		continue;
// 			    	}
// 			    	if let Ok(_) = lock.try_send(1) {
// 			    		wg.send(1);
// 			    		work_channel.send(Some(core.clone()));
// 			    		core.pop();
// 			    		continue;
// 			    	}
// 			    	root_pointer += 1;
// 				},
// 				None => {
// 					core.pop();
// 					if root_pointer == 0 {
// 						break;
// 					}
// 					root_pointer -= 1;
// 				}
// 			}
// 		}
// 		lock.recv();
// 		wg.send(-1);
// 	}
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