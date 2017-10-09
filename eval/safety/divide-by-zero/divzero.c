#include <octopos.h>

void main_ilet(claim_t claim) {

    int valid = 5 / 1;
    printf("Valid: %d\n", valid);

    int invalid = 5 / 0;
    printf("Invalid: %d\n", invalid);

    shutdown(0);
}