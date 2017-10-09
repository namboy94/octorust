#![no_std]

extern crate octolib;
use octolib::helper::printer::print_one;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let slice = [0; 1000];
    print_one("Valid(0):      %d\n\0", slice[0]);
    print_one("Valid(999):    %d\n\0", slice[999]);
    print_one("Invalid(1000): %d\n\0", slice[1000]);

}
