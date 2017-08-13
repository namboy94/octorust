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

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(2, 5);
	constraints.set_tile_shareable(true);

	let mut claim = AgentClaim::new(constraints)


	unsafe {printf("main ilet\n* Creating Constraints\n\0".as_ptr());}
    let mut myConstr = agent_constr_create();

    unsafe{ printf("* Setting Constraints\n\0".as_ptr()); }
    agent_constr_set_quantity(myConstr, 2, 5, 0);   // min 2, max 5, type 0
    agent_constr_set_tile_shareable(myConstr, 1);
    //agent_constr_set_notontile(myConstr, 0);

    let i = 0;

    unsafe {printf("* Run %d: Invading with new Agent C=%p\n\0".as_ptr(), i, myConstr);}
    let mut myClaim = agent_claim_invade(ptr::null_mut(), myConstr);

    if myClaim.is_null() {
        unsafe{printf("Invade operation unsuccessful.\n\0".as_ptr())}
		shutdown(1);
    }

	unsafe {printf("* Returned Claim:\n\0".as_ptr())};
    agent_claim_print(myClaim);

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

	let mut sync = simple_signal {padding: [0; 64]};
	simple_signal_init(&mut sync, agent_claim_get_pecount(myClaim) as usize);

	unsafe {
		for tile in 0..get_tile_count() {
			let pes = agent_claim_get_pecount_tile_type(myClaim, tile as u8, 0);
			if pes != 0 {
				// Type = 0 ^= RISC

				let mut pClaim = agent_claim_get_proxyclaim_tile_type(myClaim, tile as i32, 0);
				printf("* Got Proxy Claim %p\n\0".as_ptr(), pClaim);

				proxy_infect_with_ilet(pClaim, ILetFunc, pes, &mut sync as *mut _ as *mut c_void);
				printf("Infecting %d Ilets on Tile %d\n\0".as_ptr(), pes, tile);
			}
		}
	}

	unsafe { printf("Waiting on Signal %p...\n\0".as_ptr(), &sync);}
	simple_signal_wait(&mut sync);
	unsafe{printf("All Signals received!\n\0".as_ptr());}


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