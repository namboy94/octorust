#![no_std]

extern crate octolib;
use octolib::helloworld;

#[no_mangle]
pub extern "C" fn main_rust_ilet(claim: u8) {
    helloworld();
}