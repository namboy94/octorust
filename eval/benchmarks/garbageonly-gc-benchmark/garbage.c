#include <octopos.h>
#include <stdio.h>
#include <time.h>


void main_ilet(claim_t claim) {

    int GARBAGE_SIZE = 1000;
    int ITERATIONS = 100000;

    for (int i = 0; i < ITERATIONS; i++) {

    char (*arr)[256]=malloc(512*256);

        int *array = (int*) malloc(GARBAGE_SIZE * sizeof(int));
        for (int j = 0; j < GARBAGE_SIZE; j++) {
            array[j] = 0;
        }
		free(array);

	}

    shutdown(0);
}
