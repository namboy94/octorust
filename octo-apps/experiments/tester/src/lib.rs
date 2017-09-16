// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;
pub type ilet_func = fn(arg1: *mut c_void);

extern {
	fn printf(s: *const u8, ...);
	fn shutdown(code: usize);
	fn handle_closure(func: ilet_func);
}
use octolib::octo_types::c_void;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
	unsafe {
		let clos = |x: *mut c_void| { printf("HELLO WORLD!\n\0".as_ptr()); };
		handle_closure(clos);
		handle_closure(ilet);
	}
}

extern "C" fn ilet(data: *mut c_void) {
	unsafe {
		printf("HELLO WORLD!\n\0".as_ptr());
		reply_signal(data);
	}
}