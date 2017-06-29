#![no_std]

extern crate octolib;
use octolib::helper::printer::{newline, print_text};
use octolib::octo_bindings::octo_guest::*;
mod tests;
use tests::octo_tile::test_octo_tile;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
    print_text("Starting Testing\0");
    newline();

    test_octo_tile();

    print_text("Finished Testing\0");
    newline();

    shutdown(0);
}
