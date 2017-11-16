/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octorust.

octorust is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octorust is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octorust.  If not, see <http://www.gnu.org/licenses/>.
*/

#![no_std]

extern crate libc;
extern crate octolib;
use libc::c_void;
use octolib::helper::printer::*;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let pes = 8;
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
