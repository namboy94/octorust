#include <octopos.h>
#include <stdio.h>
#include <stdlib.h>


void main_ilet(claim_t claim) {

    int GARBAGE_SIZE = 5000;
    int ITERATIONS = 1000000;

    for (int i = 0; i < ITERATIONS; i++) {

        int *array = (int*) malloc(GARBAGE_SIZE * sizeof(int));
        for (int j = 0; j < GARBAGE_SIZE; j++) {
            array[j] = 0;
        }
		free(array);

	}

    printf("Done");
    shutdown(0);
}
