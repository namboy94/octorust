// The main OctoPOS header.
#include <octopos.h>

// C-library headers. Note that OctoPOS supports only a subset of the C runtime
// library.
#include <stdio.h>
#include <stdlib.h>

// Declaration of two i-let functions. See the bottom of the file for their
// implementation.
static void remoteILetFunc(void *arg);
static void doSignal(void *arg);

// main_ilet() is the main entry point of the program.
// The application is started directly after bootup and runs on a single claim
// consisting of exactly one processing element.
// Note the 'extern "C"' declaration, which is necessary in C++ to prevent the
// compiler from mangling the symbol name.
extern "C" void main_ilet(claim_t claim) {

	// Be friendly and greet the user.
	printf("Hello from tile %u!\n", get_tile_id());

	// Now let's invade three cores on tile 1.
	// The proxy_invade() call works asynchronously - we can continue doing
	// stuff while the request is being processed.
	invade_future future;
	if (proxy_invade(1, &future, 3) != 0) {
		fputs("proxy_invade failed - does tile 1 even exist?\n", stderr);
		abort();
	}

	// In theory, we could do arbitrary stuff between proxy_invade() and
	// invade_future_force(). In this sample application, we don't.

	// Now let's go see if the invasion worked. If it did, we get a proxy claim
	// to which we can send i-lets to be executed on tile 1.
	proxy_claim_t pc = invade_future_force(&future);
	if (pc == 0) {
		fputs("invade_future_force failed - looks like 3 free cores are not "
		      "available on tile 1.\n", stderr);
		abort();
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
	// returning from main_ilet() will not terminate the application!
	guest_shutdown();
}

// This is the function we execute on tile 1.
static void remoteILetFunc(void *arg) {

	// Be friendly and greet the user.
	printf("Hello from tile %u!\n", get_tile_id());

	// We want to tell main_ilet() that we have finished doing what we wanted to
	// do. For that purpose, we have been given a pointer to a simple_signal
	// structure (arg). But - and this is the tricky part - we cannot access
	// that data structure directly because it resides in another TLM, of which
	// we have no coherent view!
	// So we need to create another i-let, send that i-let back to the tile
	// where we came from, and let it care of doing the signalling for us.
	simple_ilet reply;
	simple_ilet_init(&reply, doSignal, arg);
	dispatch_claim_send_reply(&reply);
}

// This is the i-let that notifies main_ilet() that the execution on tile 1 has
// finished.
static void doSignal(void *arg) {
	simple_signal_signal(static_cast<simple_signal *>(arg));
}
