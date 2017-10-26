#include <octopos.h>
#include <stdio.h>

void signaler(void* sig) {
    simple_signal* s = (simple_signal*)(sig);
    simple_signal_signal_and_exit(s);
}

void ILetFunc(void *signal) {
    printf("Hello World!\n");
    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    dispatch_claim_send_reply(&answer);
}

void main_ilet(claim_t claim) {

    constraints_t myConstr = agent_constr_create();
    agent_constr_set_quantity(myConstr, 4, 4, 0);
    agent_constr_set_tile_shareable(myConstr, 1);
    agentclaim_t myClaim = agent_claim_invade(0, myConstr);

    simple_signal sync;
    simple_signal_init(&sync, agent_claim_get_pecount(myClaim));

    for (int tile=0; tile < get_tile_count(); tile++) {
        int pes=agent_claim_get_pecount_tile_type(myClaim,  tile, 0);
        if (pes) {
            proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(myClaim, tile, 0);

            simple_ilet ILet[pes];
            for (int i = 0; i < pes; ++i) {
                simple_ilet_init(&ILet[i], ILetFunc, &sync);
            }

            proxy_infect(pClaim, &ILet[0], pes);
        }
    }

    simple_signal_wait(&sync);
    shutdown(0);

}
