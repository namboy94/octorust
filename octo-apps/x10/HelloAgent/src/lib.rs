// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern { fn printf(s: *const u8, ...); }

use core::ptr;
use octolib::helper::printer::print_text;
use octolib::octo_types::{c_void};
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_guest::shutdown;
use octolib::octo_structs::simple_signal;
use octolib::octo_signal::simple_signal_signal_and_exit;
use octolib::octo_dispatch_claim::dispatch_claim_send_reply;
use octolib::octo_structs::simple_ilet;
use octolib::octo_tile::get_tile_id;
use octolib::octo_tile::get_cpu_id;
use octolib::octo_ilet::simple_ilet_init;




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

	claim.reinvade();

	let mut new_constraints = Constraints::new();
	new_constraints.set_pe_quantity(1, 1);
	// new_constraints.set_pe_quantity(6, 7);  Tile 1 died with signal SIGSEGV
	new_constraints.set_tile_shareable(true);

	// claim.reinvade_with_constraints(new_constraints); // Segmentation fault (core dumped)

	claim.infect(last_ilet);

}

extern "C" fn ilet_function(arg: *mut c_void) {
	print_text("Hello World!\n\0");
}

extern "C" fn last_ilet(arg: *mut c_void) {
	print_text("Goodbye World!\n\0");
	shutdown(0);
}

pub extern "C" fn signal_ilet(signal: *mut c_void) {

	unsafe {
		printf("iLet on tile %u running on cpu %u with parameter %p\n\0".as_ptr(), get_tile_id(), get_cpu_id(), signal);
	}

	let mut answer: simple_ilet = simple_ilet { padding: [0; 32] };
	simple_ilet_init(&mut answer, signaler, signal);

	unsafe {
		printf("Sending reply...\n\0".as_ptr());
	}

    dispatch_claim_send_reply(&mut answer);
}

pub extern "C" fn signaler(sig: *mut c_void) {

	let mut s = sig as *mut simple_signal;
	unsafe {
		printf("Signalling Signal %p\n\0".as_ptr(), s);
	}
    simple_signal_signal_and_exit(s);
}