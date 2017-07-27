// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use core::ptr;
use octolib::helper::printer::print_text;
use octolib::octo_types::{c_void};
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_guest::shutdown;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(3, 4);
    constraints.set_tile_shareable(true);

	let claimOne = AgentClaim::new(constraints);
	// let claimTwo = AgentClaim::new(constraints);

	claimOne.infect(ilet_function);
	// claim.infect(ilet_function);
	shutdown(0);
}

extern "C" fn ilet_function(arg: *mut c_void) {
	print_text("Hello World!\n\0");
}
