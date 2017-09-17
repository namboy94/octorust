// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern { fn printf(s: *const u8, ...); }
use core::ptr;
use octolib::octo_types::c_void;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);

	fn function(params: *mut c_void) {
		unsafe { printf("Hello World (fn)%d\n\0".as_ptr(), params) }
	}

	let closure = |params: *mut c_void| {
		unsafe { printf("Hello World (closure)\n\0".as_ptr()) }
	};

	let mut x = 0;
	let modifying_closure = |params: *mut c_void| {
		x = x + 1;
		unsafe { printf("Hello World (closure:%d)\n\0".as_ptr(), x) }
	};

	let data = [ptr::null_mut(), 1000 as *mut c_void, ptr::null_mut(), ptr::null_mut()];

	claim.infect(function, Some(&data));
	claim.infect(closure, None);
	claim.infect(modifying_closure, None);

}
