#![no_std]

extern crate libc;
extern crate octolib;
use libc::c_void;
use octolib::helper::printer::*;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let mut ilet = |params: *mut c_void| {
    	print("Hello World\n\0"); 
    };
    let mut constr = Constraints::new(4, 4);
    let mut agent = AgentClaim::new(constr);
    agent.infect(ilet, None);

}