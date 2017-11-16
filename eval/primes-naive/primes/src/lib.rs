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

extern crate octolib;
use octolib::helper::printer::print_one;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    primes();

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