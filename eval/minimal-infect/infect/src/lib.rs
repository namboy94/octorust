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

    let mut ilet = |params: *mut c_void| {
    	print("Hello World\n\0"); 
    };
    let mut constr = Constraints::new(4, 4);
    let mut agent = AgentClaim::new(constr);
    agent.infect(ilet, None);

}