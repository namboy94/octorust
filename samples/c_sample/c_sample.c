#include <stdio.h>
#include <stdint.h>
#include <octopos.h>

void main_ilet(uint8_t claim_t) {
    printf("Hello from tile %d!\n", get_tile_id());
    shutdown(0);
}