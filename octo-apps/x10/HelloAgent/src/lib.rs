// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use core::ptr;
use octolib::helper::printer::print_text;
use octolib::octo_types::{c_void};
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_guest::shutdown;
use octolib::octo_structs::simple_signal;
use octolib::octo_signal::simple_signal_signal_and_exit;



#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	normal_infect();
	//signal_infect()

}

fn signal_infect() {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(1, 1);
    constraints.set_tile_shareable(true);

	let mut claim = AgentClaim::new(constraints);
	claim.print_size();
	claim.set_verbose(true);

	let signal = claim.infect_signal_wait(signal_ilet);
	// claim.reinvade();
	// claim.infect(ilet_function);

	shutdown(0);

}

fn normal_infect() {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(1, 1);
	// constraints.set_pe_quantity(3, 4);  Tile 1 died with signal SIGSEGV
    constraints.set_tile_shareable(true);

	let mut claim = AgentClaim::new(constraints);
	claim.print_size();
	claim.set_verbose(true);

	claim.infect(ilet_function);
	claim.reinvade();
	claim.infect(ilet_function);


	// claim.reinvade();  Fatal error: Retreating Claim which doesn't belong to any Agent.

	let mut new_constraints = Constraints::new();
	new_constraints.set_pe_quantity(1, 1);
	// new_constraints.set_pe_quantity(6, 7);  Tile 1 died with signal SIGSEGV
	new_constraints.set_tile_shareable(true);

	// claim.reinvade_with_constraints(new_constraints);  Segmentation fault (core dumped)

	claim.infect(last_ilet);

}

extern "C" fn ilet_function(arg: *mut c_void) {
	print_text("Hello World!\n\0");
}

extern "C" fn last_ilet(arg: *mut c_void) {
	print_text("Goodbye World!\n\0");
	shutdown(0);
}

extern "C" fn signal_ilet(arg: *mut c_void) {
	simple_signal_signal_and_exit(arg as *mut simple_signal);
}