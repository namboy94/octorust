#![no_std]

extern crate libc;
extern crate octolib;
use libc::c_void;
use octolib::helper::printer::*;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let pes = 10;
    let limit = 500000;
    let mut prime_array = [false; 500000 - 2];

    let ids = [
        0 as *mut c_void,
        1 as *mut c_void,
        2 as *mut c_void,
        3 as *mut c_void,
        4 as *mut c_void,
        5 as *mut c_void,
        6 as *mut c_void,
        7 as *mut c_void,
        8 as *mut c_void,
        9 as *mut c_void
    ];

    let mut constr = Constraints::new(pes, pes);
    let mut agent = AgentClaim::new(constr);

    agent.infect(|params: *mut c_void| {

        let mut id = params as usize;

        let step: usize = limit / pes as usize;
        let mut lower_limit = id * step + 1;
        let mut upper_limit = (id + 1) * step;

        if lower_limit == 1 {
            lower_limit += 1;
        }

        if id == ((pes - 1) as usize) {
            upper_limit = limit;
        }

        for i in lower_limit..upper_limit {
            prime_array[(i - 2) as usize] = is_prime(i);
        }
    }, Some(&ids));

    for i in 2..limit {
        if prime_array[(i - 2) as usize] {
            print_one("%d\n\0", i);
        }
    }

}

fn is_prime(value: usize) -> bool {
    for i in 2..value {
        if value % i == 0 {
            return false;
        }
    }
    return true;
}
