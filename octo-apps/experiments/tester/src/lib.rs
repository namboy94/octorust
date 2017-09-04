// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn shutdown(code: usize);
}

use octolib::octo_types::c_void;
use octolib::improvements::prototypes::*;
use octolib::improvements::claim::*;
use octolib::improvements::constraints::*;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
	// test_prod();
	test_prototypes();
}

fn test_prototypes() {

	unsafe {
		let constr = ConstraintsPrototype::new(1, 4);
		let agent = ClaimPrototype::new(constr);

		for i in 0..10 {
			agent.reinvade(); // Reinvade 10 times
		}
		agent.infect(ilet);
		shutdown(0);
	}

}

fn test_prod() {

	let constr = Constraints::new(3);
	let agent = AgentClaim::new(constr);

	for i in 0..10 {
		agent.reinvade(); // Reinvade 10 times
	}
}

extern "C" fn ilet(data: *mut c_void) {
	unsafe {
		printf("HELLO WORLD!\n\0".as_ptr());
		reply_signal(data);
	}
}