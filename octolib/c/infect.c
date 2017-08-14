#include <stdio.h>
#include "octopos.h"

/**
 * This function allows Rust to conveniently initialize Ilets for a given
 * number of processing elements. After the Ilets are initialized, a
 * proxy_infect will be done using these ilets
 *
 * This is done in C because rust does not allow uninitialized arrays like
 *     simple_ilet ilets[];
 * and using malloc to get around this issue has been proven to be difficult
 * due to conflicting type sizes in rust and C.
 *
 * @param claim: The claim on which to infect using the ilets
 * @param ilet_func: The C function to execute on the Ilets
 * @param pes: The Amount of processing elements on which to execute
 * @param param: Parameter for the ilet function
 */
void proxy_infect_with_ilet(agentclaim_t claim, void (*ilet_func) (void*), int pes, void* param) {
    simple_ilet ilets[pes];
    for (int i = 0; i < pes; ++i) {
        simple_ilet_init(&ilets[i], ilet_func, param);
    }
    proxy_infect(claim, &ilets[0], pes);
}

// Temporary for testing purposes
void proxy_infect_with_ilet_and_signal(agentclaim_t claim, void (*ilet_func) (void*)) {
    simple_signal sync;
    simple_signal_init(&sync, agent_claim_get_pecount(claim));

    for (int tile=0; tile < get_tile_count(); tile++) {
        int pes=agent_claim_get_pecount_tile_type(claim,  tile, 0);
        if (pes) { // Type = 0 ^= RISC
            proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(claim, tile, 0);
            printf("* Got Proxy Claim %p\n", pClaim);

            simple_ilet ILet[pes];
            for (int i = 0; i < pes; ++i) {
                simple_ilet_init(&ILet[i], ilet_func, &sync);
            }

            printf("Infecting %d Ilets on Tile %d\n", pes, tile);
            proxy_infect(pClaim, &ILet[0], pes);
        }
    }

    printf("Waiting on Signal %p...\n", &sync);
    simple_signal_wait(&sync);
    printf("All Signals received!\n");
}

void reinvade_helper(agentclaim_t claim) {
    int status = agent_claim_reinvade(claim);
    if (status == -1) {
        printf("* Reinvade Failed\n");
    } else {
        printf("* Reinvade Successful\n");
    }
}