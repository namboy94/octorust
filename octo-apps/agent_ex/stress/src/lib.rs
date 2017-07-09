#![no_std]

extern crate octolib;
extern { fn printf(s: *const u8, ...); }
use core::ptr;
use octolib::octo_types::*;
use octolib::helper::printer::*;
use octolib::octo_agent::*;
use octolib::octo_guest::*;

const parnum: i32 = 2;
static mut globCounter: [i32; parnum as usize] = [-1; parnum as usize];

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    print_text("main ilet!\n\0");
    let initialClaim: agentclaim_t = agent_claim_get_initial(claim);
    unsafe { printf("** Invading %d Claims for Stresstest:\n\0".as_ptr(), parnum) }

    // TODO find way to initialize this in a more intelligent way
    let mut myConstr: [constraints_t; parnum as usize] = [ptr::null_mut(); parnum as usize];
    let mut myClaim: [agentclaim_t; parnum as usize] = [ptr::null_mut(); parnum as usize];

    for i in 0..parnum as usize {

        unsafe { globCounter[i] = 0; }
        myConstr[i] = agent_constr_create();
        agent_constr_set_quantity(myConstr[i], 1, 1, 0);   // min 2, max 5, type 0
        agent_constr_set_tile_shareable(myConstr[i], 1);
        agent_constr_set_appnumber(myConstr[i], i as i32);
        // myClaim[i] = agent_claim_invade(ptr::null_mut(), myConstr[i]);



        //if !myClaim[i] {
        //    printf("Invade operation unsuccessful.\0");
        //    abort();
        //}

        //printf("* Returned Claim %d of Size %d\n\0", i, agent_claim_get_pecount(myClaim[i]));
    }






    shutdown(0);

}


