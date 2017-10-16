#![feature(lang_items, libc)]
#![no_std]
#![no_main]

extern crate libc;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

// External C functions
// OctoPOS functions must be defined here directly, octolib conveninece methods are currently not
// available
extern {

    fn printf(s: *const u8, ...);
    fn shutdown(u: u32);
    fn get_tile_id() -> u32;

}

#[no_mangle]
pub extern "C" fn main_ilet(claim: u8){
	unsafe { printf("Hello World!\nTile ID: %d\n\0".as_ptr(), get_tile_id()); }
	unsafe { shutdown(0); }
}