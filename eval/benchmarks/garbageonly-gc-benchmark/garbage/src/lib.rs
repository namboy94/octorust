#![no_std]

extern crate octolib;
use octolib::helper::printer::print;
use octolib::helper::printer::print_one;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    garbage();

}

fn garbage() {

    let x = [0; 10];
    print_one("%d\n\0", x[0]);
    //print_one("%d\n\0", x[10]);
    5 / 0;

}
