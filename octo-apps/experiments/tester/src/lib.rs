// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern { fn printf(s: *const u8, ...); }
use octolib::octo_types::c_void;
use octolib::octo_guest::shutdown;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);

	let mut x = 0;
	let closure = |p: *mut c_void| { x = x + 1; unsafe { printf("%d\n\0".as_ptr(), x) }; reply_signal(p) };

	claim.infect_closure(closure);

	/*
	claim.set_verbose(true);
	claim.infect_closure(closure);
	claim.reinvade(None);

	claim.infect(ilet);
	claim.reinvade(Some(Constraints::new(6, 7)));

	claim.infect(ilet);
	*/
	// Implicit retreat
	// Shutdown handled by octorust
}
