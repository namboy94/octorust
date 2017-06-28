// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
// Based on the drr-demo application found in octopos-app-dev

// Disables the Rust Standard Library. Since the Rust Standard Library
// does not currently work on SPARC/LEON architectures, this is required.
// In theory, this may be omitted if targeting x86guest or x64native
#![no_std]

// Tells rust to include the octolib dependency
extern crate octolib;

// Tells rust which octolib functions should be accessible from this file.
use octolib::helper::printer::*;
use octolib::octo_bindings::octo_guest::guest_shutdown;
use octolib::octo_bindings::octo_tile::get_tile_id;

// main_rust_ilet() is the main entry point of the program.
// The application is started directly after bootup and runs on a single claim
// consisting of exactly one processing element.
// Note the 'extern "C"' and #[no_mangle] declaration, which are
// necessary in Rust to prevent the compiler from mangling the symbol name.
#[no_mangle]
pub extern "C" fn main_rust_ilet(claim: u8) {

    // Be friendly and greet the user.
    // Currently, this is a bit hacky since Rust's own println
    // can't be used without the standard library
    print_text("Hello from tile \0");
    print_u32(get_tile_id());
    print_text("!\n\0");

    // Now let's invade three cores on tile 1.
	// The proxy_invade() call works asynchronously - we can continue doing
	// stuff while the request is being processed.
	let future: invade_future;
	if (proxy_invade(1, &future, 3) != 0) {
		fputs("proxy_invade failed - does tile 1 even exist?\n", stderr);
		abort();
	}

    // Shut down the system.
	// If we don't do this, the system will keep running forever. Simply
	// returning from main_rust_ilet() will not terminate the application!
	guest_shutdown();

}