#![no_std]

extern crate octolib;
use octolib::octo_bindings::octo_types::*;
use octolib::octo_bindings::octo_agent::{agent_constr_create};

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    const parnum: usize = 2;
    let globCounter: [i32; parnum];

    let stressConstraints: constraints_t = agent_constr_create();

    //agent_constr_set_quantity(stressConstraints, 1, 1+rand()%15, 0);
    //agent_constr_set_tile_shareable(stressConstraints, 1);
    //agent_constr_set_appnumber(stressConstraints, (int)parm);

    // creates an agentclaim_t object!
    //let stressClaim: agentclaim_t = agent_claim_invade(NULL, stressConstraints);

    //if (!stressClaim) {
    //    fprintf(stderr, "Invade operation unsuccessful.");
    //    abort();
    //}


}


