#include <octopos.h>
#include <stdio.h>

void main_ilet(claim_t claim) {

    int LIMIT = 1690000;
    int removed[1690000 - 2] = {};
    int root = 1300;

    for (int i = 2; i < LIMIT; i++) {
        removed[i - 2] = 0;
    }

    for (int i = 2; i < root; i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
        for (int j = i * i; j < LIMIT; j += i) {
            removed[j - 2] = 1;
        }
    }

    for (int i = root + 1; i < LIMIT; i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
    }
    shutdown(0);
}
