#![feature(intrinsics, lang_items)]
#![crate_type="lib"]
#![no_std]

// enable external C functions
extern crate libc;

// link the project's submodules
pub mod octo_bindings;

// Usually in std, must be defined for an executable file
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"]#[no_mangle]  fn panic_fmt() -> ! { loop {} } // Must not be mangled!

extern {

    fn printf(s: *const u8, ...);
    #[link_name="get_tile_id"] fn __get_tile_id() -> u32;

}

pub fn get_tile_id() -> u32 {
    return unsafe { __get_tile_id() }
}
pub fn helloworld(){ unsafe { printf("Hello World".as_ptr()); } }