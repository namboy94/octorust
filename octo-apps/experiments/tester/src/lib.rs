// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn shutdown(code: usize);
}

use octolib::octo_types::c_void;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
	test();
}

fn test() {

	let constr = Constraints::new(4);
	let mut agent = AgentClaim::new(constr);
	agent.set_verbose(true);


	agent.infect(ilet);
	for i in 0..10 {
		agent.reinvade(); // Reinvade 10 times
	}
	agent.infect(ilet);
	for i in 0..10 {
		agent.reinvade(); // Reinvade 10 times
	}
	agent.infect(ilet);
	agent.reinvade_with_constraints(Constraints::new(6));
	agent.infect(ilet);
	agent.reinvade_with_constraints(Constraints::new(8));
	agent.infect(ilet);

	for i in 1..10 {
		agent.reinvade_with_constraints(Constraints::new(i));
	}
	agent.infect(ilet);

	unsafe{shutdown(0)};
}

extern "C" fn ilet(data: *mut c_void) {
	unsafe {
		printf("HELLO WORLD!\n\0".as_ptr());
		reply_signal(data);
	}
}