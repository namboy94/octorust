#![no_std]

extern crate octolib;
use octolib::helper::printer::print;
use octolib::helper::printer::print_one;

extern { fn sqrt(d: i32) -> f32; }

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    eratosthenes_primes();

}

fn eratosthenes_primes() {

    let LIMIT = 1000000;
    let mut removed = [false; 1000000 - 2];
    let root = 1000; // No sqrt without std
    unsafe{ sqrt(10000) }; // Call C sqrt to make things fair

    for i in 2..root {
        if !removed[i - 2] {
            print_one("%d\n\0", i);
        }
        let mut j = i * i;
        while j < LIMIT {
            removed[j - 2] = true;
            j += i;
        }
    }

    for i in (root + 1)..LIMIT {
        if !removed[i - 2] {
            print_one("%d\n\0", i);
        }
    }
}
