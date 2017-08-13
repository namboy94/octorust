// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_types::*;

extern {
	fn linkertester();
}


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	unsafe {
		linkertester();
	}

}
