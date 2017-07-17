// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
// Based on the drr-demo application found in octopos-app-dev

// Disables the Rust Standard Library. Since the Rust Standard Library
// does not currently work on SPARC/LEON architectures, this is required.
// In theory, this may be omitted if targeting x86guest or x64native
#![no_std]

// Tells rust to include the octolib dependency
extern crate octolib;

// TODO integrate in octolib
extern {
    fn abort();
}

// Tells rust which octolib functions should be accessible from this file.
use octolib::helper::printer::*;
use octolib::octo_guest::*;
use octolib::octo_tile::*;
use octolib::octo_proxy_claim::*;
use octolib::octo_types::*;
use octolib::octo_signal::*;
use octolib::octo_ilet::*;
use octolib::octo_dispatch_claim::*;
use octolib::octo_structs::*;
use octolib::octo_improvements::*;
use core::ptr;

// main_rust_ilet() is the main entry point of the program.
// The application is started directly after bootup and runs on a single claim
// consisting of exactly one processing element.
// Note the 'extern "C"' and #[no_mangle] declaration, which are
// necessary in Rust to prevent the compiler from mangling the symbol name.
#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

    // Be friendly and greet the user.
    // Currently, this is a bit hacky since Rust's own println
    // can't be used without the standard library
    print_text("Hello from tile \0");
    print_u32(get_tile_id());
    print_text("!\n\0");

    // Now let's invade three cores on tile 1.
	// The proxy_invade() call works asynchronously - we can continue doing
	// stuff while the request is being processed.
	let invade_result = initialize_proxy_invade(1, 3);
	let mut future = invade_result.0;

	if !invade_result.1 {
	    print_text("proxy_invade failed - does tile 1 even exist?\n\0");
		unsafe { abort(); }
	}

	// In theory, we could do arbitrary stuff between proxy_invade() and
	// invade_future_force(). In this sample application, we don't.

	// Now let's go see if the invasion worked. If it did, we get a proxy claim
	// to which we can send i-lets to be executed on tile 1.
	let pc: proxy_claim_t = invade_future_force(&mut future);
	if pc == ptr::null_mut() as *mut c_void {
	    print_text("invade_future_force failed - looks like 3 free cores are \
	                not available on tile 1.\n");
	    unsafe { abort(); }
    }

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
	let mut signal = simple_signal { padding: [0; 64] };
	simple_signal_init(&mut signal, 1);

	// Create the i-let we want to execute on tile 1, and pass a pointer to the
	// signal as argument.
	let mut iLet: simple_ilet = simple_ilet { padding: [0; 32] };
	simple_ilet_init(&mut iLet, remoteILetFunc, &mut signal as *mut _ as *mut c_void);
	proxy_infect(pc, &mut iLet, 1);

	// Now wait until the remote i-let has notified us of its completion.
	simple_signal_wait(&mut signal);

    // Shut down the system.
	// If we don't do this, the system will keep running forever. Simply
	// returning from main_rust_ilet() will not terminate the application!
	guest_shutdown();

}


// This is the function we execute on tile 1.
extern "C" fn remoteILetFunc(arg: *mut c_void) {

	// Be friendly and greet the user.
	print_text("Hello from tile \0");
    print_u32(get_tile_id());
    print_text("!\n\0");

	// We want to tell main_ilet() that we have finished doing what we wanted to
	// do. For that purpose, we have been given a pointer to a simple_signal
	// structure (arg). But - and this is the tricky part - we cannot access
	// that data structure directly because it resides in another TLM, of which
	// we have no coherent view!
	// So we need to create another i-let, send that i-let back to the tile
	// where we came from, and let it care of doing the signalling for us.
	let mut reply: simple_ilet = simple_ilet { padding: [0; 32] };
	simple_ilet_init(&mut reply, doSignal, arg);
	dispatch_claim_send_reply(&mut reply);
}

// This is the i-let that notifies main_ilet() that the execution on tile 1 has
// finished.
extern "C" fn doSignal(arg: *mut c_void) {

	simple_signal_signal(arg as *mut simple_signal);
}
