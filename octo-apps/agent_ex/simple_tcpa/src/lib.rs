#![no_std]

extern { fn printf(s: *const u8, ...); }

extern crate octolib;
use core::ptr;
use octolib::octo_types::*;
use octolib::octo_agent::*;
use octolib::octo_structs::*;
use octolib::octo_ilet::*;
use octolib::octo_tile::*;
use octolib::octo_proxy_claim::*;
use octolib::octo_guest::*;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    // Choose which way to run:
    // c_api();
    improved();

    return;
}

fn c_api() {

    let my_constr = agent_constr_create();
    agent_constr_set_quantity(my_constr, 1, 1, 0);

    let my_claim = agent_claim_invade(ptr::null_mut(), my_constr);
    agent_claim_print(my_claim);

    unsafe {
        printf("* Returned Claim of Size %d\n\0".as_ptr(), agent_claim_get_pecount(my_claim));
    }

    let mut my_ilet: simple_ilet = simple_ilet { padding: [0; 32] };
    simple_ilet_init(&mut my_ilet, hello_world_ilet, ptr::null_mut());

    for tile in 0..get_tile_count() {
        let pes = agent_claim_get_pecount_tile_type(my_claim, tile as u8, 0);

        if pes > 0 {
            let proxy_claim = agent_claim_get_proxyclaim_tile_type(my_claim, tile as i32, 0);
            unsafe { printf("* Got Proxy Claim %p\n\0".as_ptr(), proxy_claim); }
            proxy_infect(proxy_claim, &mut my_ilet, pes as u32);
        }

    }

}


fn improved() {

    let mut constraints = Constraints::new();
    constraints.set_pe_quantity(1, 1);

    let mut claim = AgentClaim::new(constraints);
    claim.print_size();
    claim.infect(hello_world_ilet);

}

extern "C" fn hello_world_ilet(param: *mut c_void) {
    unsafe {
        printf("Hello World from iLet on tile %u running on cpu %u with parameter %p\n\0".as_ptr(), get_tile_id(), get_cpu_id(), param);
    }
    guest_shutdown();
}