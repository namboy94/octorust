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
    //simple_ilet ilets[pes];
    simple_ilet* ilets = malloc(pes * sizeof(simple_ilet));
    printf("C SIZEOF ILETS%d, PES:%d, TOTAL:%d\n", sizeof(simple_ilet), pes, pes * sizeof(simple_ilet));
    for (int i = 0; i < pes; ++i) {
        simple_ilet_init(&ilets[i], ilet_func, param);
    }
    proxy_infect(claim, &ilets[0], pes);
    free(ilets);
}
