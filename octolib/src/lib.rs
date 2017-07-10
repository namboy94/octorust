#![feature(intrinsics, lang_items)]
#![crate_type="staticlib"]
#![no_std]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// enable external C functions
extern crate libc;

// link the project's submodules
pub mod helper;
pub mod constants;
pub mod octo_tile;
pub mod octo_guest;
pub mod octo_types;
pub mod octo_agent;
pub mod octo_proxy_claim;
pub mod octo_ilet;
pub mod octo_cas;
pub mod octo_ldma;
pub mod octo_syscall_future;
pub mod octo_dispatch_claim;
pub mod octo_leon;
pub mod octo_app;
pub mod octo_cilk_support;
pub mod octo_claim;
pub mod octo_clock;
pub mod octo_debug;
pub mod octo_vga;
pub mod octo_pull_dma;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[no_mangle] pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) { }
