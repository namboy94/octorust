#![no_std]

extern crate octolib;
use octolib::helper::printer::print_one;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    primes();

}

fn primes() {

    let LIMIT = 10000;

    for i in 2..LIMIT {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            print_one("%d\n\0", i);
        }
    }
}