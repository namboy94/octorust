// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
}

use octolib::improvements::prototypes::*;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	unsafe {
		let constr = ConstraintsPrototype::new(1, 2);
		let agent = ClaimPrototype::new(constr);

		for i in 0..10 {
			agent.reinvade(); // Reinvade 10 times
		}
	}
}