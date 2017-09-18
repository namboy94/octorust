// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use octolib::octo_types::c_void;
use octolib::helper::printer::print;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	fn hello(param: *mut c_void) { print("Hello World!\n\0") }

	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);
	claim.set_verbose(true);

	claim.infect(hello, None);
	claim.reinvade(None);

	claim.infect(hello, None);
	claim.reinvade(Some(Constraints::new(6, 7)));

	claim.infect(hello, None);

}
