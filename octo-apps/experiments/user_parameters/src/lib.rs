// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern { fn printf(s: *const u8, ...); }
use core::ptr;
use core::slice;
use octolib::octo_types::c_void;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);

	let strings = [
		"Hello\0".as_ptr() as *mut c_void,
		"World\0".as_ptr() as *mut c_void,
		"Hallo\0".as_ptr() as *mut c_void,
		"Welt\0".as_ptr() as *mut c_void];

	claim.infect(|params: *mut c_void| {
		unsafe { printf("%s\n\0".as_ptr(), params) }
	}, Some(&strings));

	let arrays = [
		[1,2,3,4,5].as_mut_ptr() as *mut c_void,
		[2,3,4,5,6].as_mut_ptr() as *mut c_void,
		[3,4,5,6,7].as_mut_ptr() as *mut c_void,
		[4,5,6,7,8].as_mut_ptr() as *mut c_void
	];

	let sum = |params: *mut c_void| {
		unsafe {
			let slice = core::slice::from_raw_parts(params as *mut i32, 5);
			let mut result = 0;
			for i in 0..5 {
				result += slice[i];
			}
			printf("%d+%d+%d+%d+%d = %d\n\0".as_ptr(), slice[0], slice[1], slice[2], slice[3], slice[4], result);
		}
	};

	claim.infect(sum, Some(&arrays));

	let mut more_arrays = [
		[1,1,1,1,1, -1],
		[2,2,2,2,2, -1],
		[3,3,3,3,3, -1],
		[4,4,4,4,4, -1]
	];
	let indices = [0 as *mut c_void, 1 as *mut c_void, 2 as *mut c_void, 3 as *mut c_void];
	let mut total_sum = 0;

	claim.infect(|params: *mut c_void| {
		let slice = more_arrays[params as usize];
		let sum = slice[0] + slice[1] + slice[2] + slice[3] + slice[4];
		more_arrays[params as usize][5] = sum;
		total_sum += sum;
	}, Some(&indices));

	for i in 0..4 {
		unsafe {
			printf("Sum %d: %d\n\0".as_ptr(), i, more_arrays[i][5]);
		}
	}
	unsafe {
		printf("Total: %d\n\0".as_ptr(), total_sum);
	}

}
