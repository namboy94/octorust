#include <octopos.h>
#include <stdio.h>
#include <time.h>


void main_ilet(claim_t claim) {

    garbage();
    shutdown(0);
}

void garbage() {

    int test[10] = {};
    printf("%d\n", test[0]);
    printf("%d\n", test[10]);

    5 / 0;

}