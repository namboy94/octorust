#![no_std]

extern crate octolib;
use octolib::helper::printer::print;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let GARBAGE_SIZE = 10000;
    let ITERATIONS = 1000000;

    for i in 0..ITERATIONS {
        let garbage = [0; 10000]; // 10000 == GARBAGE_SIZE
    }

    print("Done\n\0");

}
