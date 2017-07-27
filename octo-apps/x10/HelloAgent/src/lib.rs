// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use core::ptr;
use octolib::helper::printer::print_text;
use octolib::octo_types::{c_void};
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_guest::guest_shutdown;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(3, 4);
    constraints.set_tile_shareable(true);

	let mut claim = AgentClaim::new(constraints);
	claim.print_size();
	claim.set_verbose(true);

	claim.infect(ilet_function);
	claim.reinvade();
	claim.infect(ilet_function);

	let mut new_constraints = Constraints::new();
	new_constraints.set_pe_quantity(6, 7);
	new_constraints.set_tile_shareable(true);

	claim.reinvade_with_constraints(new_constraints);
	claim.infect(ilet_function);

}

extern "C" fn ilet_function(arg: *mut c_void) {
	print_text("Hello World!\n\0");
	guest_shutdown();
}
