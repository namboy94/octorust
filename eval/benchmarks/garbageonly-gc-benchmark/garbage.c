#include <octopos.h>
#include <stdio.h>
#include <time.h>


void main_ilet(claim_t claim) {

    int GARBAGE_SIZE = 10000;
    int ITERATIONS = 1000000;

    for (int i = 0; i < ITERATIONS; i++) {

		int array[10000] = {}; // 10000 == GARBAGE_SIZE
		free(array);

	}

    shutdown(0);
}
