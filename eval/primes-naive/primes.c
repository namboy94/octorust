#include <octopos.h>
#include <stdio.h>

void main_ilet(claim_t claim) {

    int LIMIT = 50000;

    for (int i = 2; i < LIMIT + 1; i++) {
        int is_prime = 1;
        for (int j = 2; j < i; j++) {
            if (i % j == 0) {
                is_prime = 0;
            }
        }
        if(is_prime){
            printf("%d\n", i);
        }
    }

    shutdown(0);
}
