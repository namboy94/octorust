#![no_std]

extern crate octolib;
extern { fn printf(s: *const u8, ...); fn rand() -> i32; }
use core::ptr;
use octolib::octo_structs::*;
use octolib::octo_ilet::*;
use octolib::octo_tile::*;
use octolib::octo_types::*;
use octolib::octo_agent::*;
use octolib::octo_guest::*;
use octolib::helper::printer::*;
use octolib::octo_dispatch_claim::*;
use octolib::octo_proxy_claim::*;

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

        // Works with 2017-06-07 but not newer versions
        myClaim[i] = agent_claim_invade(ptr::null_mut(), myConstr[i]);

        if myClaim[i] == ptr::null_mut() {
            print_text("Invade operation unsuccessful.\n\0");
            shutdown(1);
        }

        unsafe {
            printf("* Returned Claim %d of Size %d\n\0".as_ptr(), i, agent_claim_get_pecount(myClaim[i]));
        }
    }

    for i in 0..parnum as usize {
        for tile in 0..get_tile_count() as i32 {

            let mut pes = agent_claim_get_pecount_tile_type(myClaim[i], tile as u8, 0);
            if pes != 0 {

                let pClaim: proxy_claim_t = agent_claim_get_proxyclaim_tile_type(myClaim[i], tile, 0);
                unsafe { printf("* Got Proxy Claim %p\n\0".as_ptr(), pClaim); }

                let mut first = simple_ilet { padding: [0; 32] };
                simple_ilet_init(&mut first, StressILet, i as *mut c_void); // parms);

                for iletnr in 1..pes {
                    let mut ilet = simple_ilet { padding: [0; 32] };
                    simple_ilet_init(&mut ilet, StressILet, i as *mut c_void); // parms);
                }

                unsafe {printf("Infecting %d Ilets on Tile %d\n\0".as_ptr(), pes, tile);}
                proxy_infect(pClaim, &mut first, pes as u32);
            }

        }
    }

    shutdown(0);

}


// This is the i-let that notifies main_ilet() that the execution on tile 1 has
// finished.
extern "C" fn StressILet(parm: *mut c_void) {

    print_text("stress\0");

    let mut stressConstraints = agent_constr_create();

    let random = unsafe { 1 + rand() % 15 };

    agent_constr_set_quantity(stressConstraints, 1, random, 0);
    agent_constr_set_tile_shareable(stressConstraints, 1);
    agent_constr_set_appnumber(stressConstraints, parm as i32);
    let mut stressClaim = agent_claim_invade(ptr::null_mut(), stressConstraints); // creates an agentclaim_t object!

    if stressClaim as usize != 0 {
        print_text("Invade operation unsuccessful.\n\0");
        shutdown(1);
    }

    print_u32(parm as u32);

    unsafe {
        globCounter[parm as usize] += 1;
        if globCounter[parm as usize] % 1000 == 0 {
            printf("* StressiLet %d got Claim of Size %d [%d]\n\0".as_ptr(), parm as usize, agent_claim_get_pecount(stressClaim), globCounter[parm as usize])
        }
    }

    agent_claim_retreat(stressClaim);
    agent_constr_delete(stressConstraints);

    let mut ILet = simple_ilet {padding: [0; 32] };
    simple_ilet_init(&mut ILet, StressILet, parm);
    infect_self_single(&mut ILet);
}