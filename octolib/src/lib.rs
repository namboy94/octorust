#![feature(intrinsics, lang_items)]
#![crate_type="staticlib"]
#![no_std]

#![allow(non_camel_case_types)]

// enable external C functions
extern crate libc;

// link the project's submodules
pub mod helper;
pub mod octo_tile;
pub mod octo_guest;
pub mod octo_types;
pub mod octo_agent;
pub mod octo_proxy_claim;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"]#[no_mangle]  fn panic_fmt() -> ! { loop {} }
#[no_mangle] pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }
