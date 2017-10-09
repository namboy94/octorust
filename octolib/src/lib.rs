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
    loop {}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }
