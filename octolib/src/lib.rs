#![feature(intrinsics, lang_items)]
#![crate_type="lib"]
#![no_std]

#![allow(non_camel_case_types)]

// enable external C functions
extern crate libc;

// link the project's submodules
pub mod octo_bindings;
pub mod helper;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"]#[no_mangle]  fn panic_fmt() -> ! { loop {} } // Must not be mangled!
