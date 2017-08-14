// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

// TODO
// OK, seems like sizes of the structs are a problem with malloc
// Idea: Make own improvements to octolib in C and import that!

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn proxy_infect_with_ilet(claim: agentclaim_t, ilet_func: extern fn(arg1: *mut c_void), pes: i32, param: *mut c_void);
}

use core::mem;
use core::ptr;

use octolib::_libc::{free, malloc};

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

	unsafe {printf("main ilet\n* Creating Constraints\n\0".as_ptr());}
	let mut constraints = Constraints::new();

	unsafe{ printf("* Setting Constraints\n\0".as_ptr()); }
	constraints.set_pe_quantity(2, 5);
	constraints.set_tile_shareable(true);

	unsafe {printf("* Run: Invading with new Agent\n\0".as_ptr());}
	let mut claim = AgentClaim::new(constraints);
	claim.set_verbose(true);

	for i in 0..10 {  // TODO change to 0..10 in due time
		unsafe{printf("* Reinvading:\n\0".as_ptr());}
        claim.reinvade();
    }

	claim.infect_signal_wait(ILetFunc);

	for i in 0..0 {  // TODO change to 0..10 in due time
		unsafe{printf("* Reinvading:\n\0".as_ptr());}
        claim.reinvade();
    }

	shutdown(0);

	/*
	This works :|


	let mut myConstr = agent_constr_create();
	agent_constr_set_quantity(myConstr, 2, 5, 0);
    agent_constr_set_tile_shareable(myConstr, 1);
	let mut myClaim = agent_claim_invade(ptr::null_mut(), myConstr);

	for i in 0..10 {
        unsafe{printf("* Reinvading:\n\0".as_ptr());}

        let ret = agent_claim_reinvade(myClaim);

        if ret == -1 {
            unsafe{printf("Reinvade operation unsuccessful.\n\0".as_ptr());}
			shutdown(1);
        }

        unsafe{printf("* Returned Claim:\n\0".as_ptr());}
        agent_claim_print(myClaim);
    }


	// */


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