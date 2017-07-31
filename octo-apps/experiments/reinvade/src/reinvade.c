# include "octopos.h"
# include <stdio.h>

void main_ilet(claim_t claim) {

    printf("main ilet\n");

    printf("* Creating Constraints\n");
    constraints_t myConstr = agent_constr_create();

    printf("* Setting Constraints\n");
    agent_constr_set_quantity(myConstr, 2, 5, 0);   // min 2, max 5, type 0
    agent_constr_set_tile_shareable(myConstr, 1);
    //agent_constr_set_notontile(myConstr, 0);
    int i=0;

    printf("* Run %d: Invading with new Agent C=%p\n", i, myConstr);
    agentclaim_t myClaim = agent_claim_invade(NULL, myConstr);

    infect_and_wait(myClaim);
    shutdown(0);

}

void signaler(void* sig) {
    simple_signal* s = (simple_signal*)(sig);
    printf("Signalling Signal %p\n",s);
    simple_signal_signal_and_exit(s);
}

void ILetFunc(void *signal) {
    printf("iLet on tile %u running on cpu %u with parameter %p\n", get_tile_id(), get_cpu_id(), signal);

    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    printf("Sending reply...\n");
    dispatch_claim_send_reply(&answer);
}


void infect_and_wait(agentclaim_t myClaim) {
    simple_signal sync;
    simple_signal_init(&sync, agent_claim_get_pecount(myClaim));

    printf("Tile Count: %d\n\0", get_tile_count());

    for (int tile=0; tile < get_tile_count(); tile++) {
        printf("Tile: %d\n\0", tile);
        int pes=agent_claim_get_pecount_tile_type(myClaim,  tile, 0);
        printf("pes: %d\n\0", pes);
        if (pes) { // Type = 0 ^= RISC

            printf("INFECTING!\n\0");

            proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(myClaim, tile, 0);
            printf("* Got Proxy Claim %p\n", pClaim);

            simple_ilet* ilets = malloc(pes * sizeof(simple_ilet));

            // simple_ilet ILet[pes];
            for (int i = 0; i < pes; ++i) {
                simple_ilet_init(&ilets[i], ILetFunc, &sync);
            }


            printf("Infecting %d Ilets on Tile %d\n", pes, tile);
            proxy_infect(pClaim, &ilets[0], pes);

            free(ilets);
        }
    }

    printf("Waiting on Signal %p...\n", &sync);
    simple_signal_wait(&sync);
    printf("All Signals received!\n");
}