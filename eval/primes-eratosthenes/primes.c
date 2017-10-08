#include <octopos.h>
#include <stdio.h>
#include <math.h>

void main_ilet(claim_t claim) {

    eratosthenes_primes();
    shutdown(0);
}

void eratosthenes_primes() {

    int LIMIT = 1000000;
    int removed[1000000 - 2] = {};

    for (int i = 2; i < LIMIT; i++) {
        removed[i - 2] = 0;
    }

    for (int i = 2; i < sqrt(LIMIT); i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
        for (int j = i * i; j < LIMIT; j += i) {
            removed[j - 2] = 1;
        }
    }

    for (int i = sqrt(LIMIT) + 1; i < LIMIT; i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
    }
}