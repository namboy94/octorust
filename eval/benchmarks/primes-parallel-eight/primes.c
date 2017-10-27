#include <octopos.h>
#include <stdio.h>

#define PES 8
#define LIMIT 500000

int primes_array[LIMIT - 2];

int is_prime(int num) {
    for (int i = 2; i < num; i++) {
        if (num % i == 0) {
            return 0;
        }
    }
    return 1;
}

void signaler(void* sig) {
    simple_signal* s = (simple_signal*)(sig);
    simple_signal_signal_and_exit(s);
}

void ILetFunc(void *signal) {

    int id = get_cpu_id() - 1;
    int step = LIMIT / PES;
    int lower_limit = id * step + 1;
    int upper_limit = (id + 1) * step;

    if (lower_limit == 1) {
        lower_limit += 1;
    }
    if (id == (PES - 1)) {
        upper_limit = LIMIT;
    }

    for (int i = lower_limit; i < upper_limit; i++) {
        primes_array[(i - 2)] = is_prime(i);
    }

    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    dispatch_claim_send_reply(&answer);
}

void main_ilet(claim_t claim) {

    constraints_t myConstr = agent_constr_create();
    agent_constr_set_quantity(myConstr, PES, PES, 0);
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

    for (int i = 2; i  < LIMIT; i++) {
        if (primes_array[i - 2] != 0) {
            printf("%d\n", i);
        }
    }

    shutdown(0);

}
