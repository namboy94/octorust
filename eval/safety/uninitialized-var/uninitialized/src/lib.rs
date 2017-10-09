#![no_std]

extern crate octolib;
use octolib::helper::printer::print_one;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let valid = 5 / 1;
    print_one("Valid: %d\n\0", valid);

    let invalid: i32;
    print_one("Invalid: %d\n\0", invalid);

}
