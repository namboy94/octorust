// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern { fn printf(s: *const u8, ...); }

use core::ptr;
use octolib::helper::printer::*;
use octolib::octo_types::*;
use octolib::improvements::constraints::*;
use octolib::improvements::claim::*;
use octolib::octo_guest::*;
use octolib::octo_structs::*;
use octolib::octo_signal::*;
use octolib::octo_tile::*;
use octolib::octo_dispatch_claim::*;
use octolib::octo_ilet::*;
use octolib::octo_proxy_claim::*;
use octolib::octo_agent::*;




#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(1, 1);
    constraints.set_tile_shareable(true);

	let mut agent = AgentClaim::new(constraints);
	agent.infect_signal_wait(ILetFunc);
	shutdown(0);

}

pub extern "C" fn ILetFunc(signal: *mut c_void) {

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