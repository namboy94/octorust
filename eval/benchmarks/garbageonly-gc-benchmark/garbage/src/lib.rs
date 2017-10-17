#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate octolib;
use octolib::helper::printer::print;
use alloc::boxed::Box;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    let GARBAGE_SIZE = 10000;
    let ITERATIONS = 1000000;

    for i in 0..ITERATIONS {
        let garbage = Box::new([0; 10000]); // 10000 == GARBAGE_SIZE
    }

    print("Done\n\0");

}
