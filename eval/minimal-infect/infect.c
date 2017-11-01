#include <octopos.h>
#include <stdio.h>

void signaler(void* sig) {
    simple_signal* s = (simple_signal*)(sig);
    simple_signal_signal_and_exit(s);
}

void ilet_fn(void *signal) {
    printf("Hello World!\n");
    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    dispatch_claim_send_reply(&answer);
}

void main_ilet(claim_t claim_id) {

    constraints_t constr = agent_constr_create();
    agent_constr_set_quantity(constr, 4, 4, 0);
    agent_constr_set_tile_shareable(constr, 1);
    agentclaim_t claim = agent_claim_invade(0, constr);

    simple_signal sync;
    simple_signal_init(&sync, agent_claim_get_pecount(claim));

    for (int tile=0; tile < get_tile_count(); tile++) {
        int pes=agent_claim_get_pecount_tile_type(claim,  tile, 0);
        if (pes) {
            proxy_claim_t p_claim = 
                agent_claim_get_proxyclaim_tile_type(
                    claim, tile, 0
                );

            simple_ilet ilets[pes];
            for (int i = 0; i < pes; ++i) {
                simple_ilet_init(&ilets[i], ilet_fn, &sync);
            }

            proxy_infect(p_claim, &ilets[0], pes);
        }
    }

    simple_signal_wait(&sync);
    agent_claim_retreat(claim);
    shutdown(0);
}