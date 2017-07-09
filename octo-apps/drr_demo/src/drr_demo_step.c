// The main OctoPOS header.
#include <octopos.h>

// C-library headers. Note that OctoPOS supports only a subset of the C runtime
// library.
#include <stdio.h>
#include <stdlib.h>


void main_ilet(claim_t claim) {

    // Be friendly and greet the user.
	printf("Hello from tile %u!\n", get_tile_id());

	// Now let's invade three cores on tile 1.
	// The proxy_invade() call works asynchronously - we can continue doing
	// stuff while the request is being processed.
	struct invade_future future;
	if (proxy_invade(1, &future, 3) != 0) {
		fputs("proxy_invade failed - does tile 1 even exist?\n", stderr);
		abort();
	}

	shutdown(0);

}