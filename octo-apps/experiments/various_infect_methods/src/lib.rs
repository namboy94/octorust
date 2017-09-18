// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use core::ptr;
use octolib::octo_types::c_void;
use octolib::helper::printer::*;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);

	fn function(param: *mut c_void) { print("Hello World! (fn)\n\0"); }
	let closure = |param: *mut c_void| { print("Hello World! (closure)\n\0") };

	let mut x = 0;
	let modifying_closure = |params: *mut c_void| {
		x = x + 1;
		print_one("Hello World! (modifying closure: %d)\n\0", x)
	};

	claim.infect(function, None);
	claim.infect(closure, None);
	claim.infect(modifying_closure, None);

}
