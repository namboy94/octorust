// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn handle_closure(func: i32, params: *mut c_void);
}
use core::ptr;
use core::mem;
use octolib::octo_types::c_void;
use octolib::octo_types::c_int;
use octolib::octo_guest::shutdown;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::closure_wrapper::*;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let mut closure = |params: *mut c_void| { unsafe { printf("Hello World!\n\0".as_ptr()) } };
	run_closure_in_c(closure, ptr::null_mut());

}