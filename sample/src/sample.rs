#![no_std]

extern crate octolib;
use octolib::{helloworld,shutdown,print_tile_id,newline};

#[no_mangle]
pub extern "C" fn main_rust_ilet(claim: u8) {
    helloworld();
    newline();
    print_tile_id();
    newline();
    shutdown(0);
}
