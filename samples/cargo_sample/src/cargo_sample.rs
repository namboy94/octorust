#![no_std]

extern crate octolib;
use octolib::helper::printer::{newline, print_text, print_u32};
use octolib::octo_tile::{get_tile_id};
use octolib::octo_guest::{shutdown};

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
    print_text("Hello World!\nTile ID: ");
    print_u32(get_tile_id());
    newline();
    shutdown(0);
}
