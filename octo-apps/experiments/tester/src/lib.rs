// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

use octolib::octo_types::c_void;
//use octolib::helper::printer::print;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let constr = Constraints::new(1, 2);
	let mut agent = AgentClaim::new(constr);

	let mut hello = |param: *mut c_void| {};
	agent.set_verbose(true);
	agent.infect(hello, None)

}
