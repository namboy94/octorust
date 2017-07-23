#![no_std]

extern crate octolib;
use octolib::helper::printer::*;
use octolib::octo_guest::*;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let ilet = |id: i32| { print_text("Hello World!\n\0"); };

    // Create Constraints

    // Invade (contraints)

    // constr = agent_constr_create()
    // agent_constr_set...(constr, ...)
    // agent_claim_invade(parentagent: agent_t, constr: constraints_t);
    // claim = agent_claim_invade(..., constr)

    // Infect (ilet)
    // Reinvade ()
    // Infect (ilet)
    // Reinvade (nw constraints)
    // Infect (ilet)
    // Retreat

    shutdown(0);

}


