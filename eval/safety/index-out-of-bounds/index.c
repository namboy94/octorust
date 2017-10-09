#include <octopos.h>

void main_ilet(claim_t claim) {

    int array[1000];
    for (int i = 0; i < 1000; i++) {
        array[i] = 1;
    }

    printf("Valid(0):      %d\n", array[0]);
    printf("Valid(999):    %d\n", array[999]);
    printf("Invalid(1000): %d\n", array[1000]);

    shutdown(0);
}