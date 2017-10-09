#![no_std]

extern crate octolib;
use octolib::helper::printer::print_one;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

}
