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
	// TODO implement invade_future, proxy_invade, abort
	// let future: invade_future;
	// if proxy_invade(1, &future, 3) != 0 {
	//     print_text("proxy_invade failed - does tile 1 even exist?\n\0");
	//	   abort();
	// }

	// In theory, we could do arbitrary stuff between proxy_invade() and
	// invade_future_force(). In this sample application, we don't.

	// Now let's go see if the invasion worked. If it did, we get a proxy claim
	// to which we can send i-lets to be executed on tile 1.
	// TODO Implement proxy_claim_t, invade_future_force
	// let pc: proxy_claim_t = invade_future_force(&future);
	// if pc == 0 {
	//	   print_text("invade_future_force failed - looks like 3 free cores are \
	//	               not available on tile 1.\n");
	//	   abort();
	// }

	// TODO BOOKMARK

	// We will send an i-let to tile 1 and then want to wait until that i-let
	// has finished executing. For this purpose, we use a signalling primitive.
	// The simple_signal construct holds an internal counter and offers two
	// operations:
	// * simple_signal_wait() suspends the calling i-let until the counter
	//   reaches zero. If the counter is already zero, it returns immediately.
	// * simple_signal_signal() decrements the counter. If the counter reaches
	//   zero, the suspended i-let is woken up.

	// Create a signal and initialise its counter with 1.
	// Note that the signal lies on the i-let's stack, in our own TLM.
	simple_signal signal;
	simple_signal_init(&signal, 1);

	// Create the i-let we want to execute on tile 1, and pass a pointer to the
	// signal as argument.
	simple_ilet iLet;
	simple_ilet_init(&iLet, remoteILetFunc, &signal);
	proxy_infect(pc, &iLet, 1);

	// Now wait until the remote i-let has notified us of its completion.
	simple_signal_wait(&signal);

    // Shut down the system.
	// If we don't do this, the system will keep running forever. Simply
	// returning from main_rust_ilet() will not terminate the application!
	guest_shutdown();

}