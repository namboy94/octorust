// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn proxy_infect_with_ilet(claim: agentclaim_t, ilet_func: extern fn(arg1: *mut c_void), pes: i32, param: *mut c_void);
}

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
use core::ptr;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	let mut myConstr = agent_constr_create();
	agent_constr_set_quantity(myConstr, 2, 5, 0);
    agent_constr_set_tile_shareable(myConstr, 1);
	let mut myClaim = agent_claim_invade(ptr::null_mut(), myConstr);
	unsafe {printf("%d\n\0".as_ptr(), agent_claim_reinvade(myClaim)) };
	// shutdown(0);


	let mut constr = Constraints::new();
	constr.set_pe_quantity(1, 1);
	let mut agent = AgentClaim::new(constr);
	agent.set_verbose(true);
	agent.reinvade()

}

pub extern "C" fn signaler(sig: *mut c_void) {
	let mut s: *mut simple_signal = sig as *mut _ as *mut simple_signal;
    unsafe{printf("Signalling Signal %p\n\0".as_ptr(),s);}
    simple_signal_signal_and_exit(s);
}

pub extern "C" fn ILetFunc(signal: *mut c_void) {

	unsafe { printf("iLet on tile %u running on cpu %u with parameter %p\n\0".as_ptr(), get_tile_id(), get_cpu_id(), signal);}

	let mut answer = simple_ilet {padding: [0; 32]};
    simple_ilet_init(&mut answer, signaler, signal);
    unsafe{printf("Sending reply...\n\0".as_ptr());}
    dispatch_claim_send_reply(&mut answer);
}