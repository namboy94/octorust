#![no_std]

extern crate libc;
extern crate octolib;
use libc::c_void;
use octolib::helper::printer::print_one;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let mut ilet = |x: *mut c_void| { print_one("test%d\n\0", 0); };

    let mut constr = Constraints::new(4, 4);
    let mut agent = AgentClaim::new(constr);

    agent.infect(ilet, None);

}

fn primes() {

    let LIMIT = 50000;

    for i in 2..LIMIT {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            print_one("%d\n\0", i);
        }
    }
}