/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

#![feature(intrinsics, lang_items, libc)]
#![crate_type="staticlib"]
#![no_std]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// #[macro_use]
// extern crate serde_derive;

// enable external C functions
extern crate libc;

// link the project's submodules
pub mod helper;
pub mod octo_structs;
pub mod octo_types;

pub mod _libc;
pub mod bindings;

// Improvements
pub mod improvements;

use helper::printer::print;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! {
    print("PANICKED!\n\0");
    unsafe {
        bindings::octo_guest::shutdown(1);
    }
    loop {}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }
