#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "octopos.h"
#include "octo_agent.h"

static void die(const char message[]) {
    fprintf(stderr, "%s: failed\n", message);
    abort();
}

void signaler(void* sig) {
/*
    simple_signal* s = (simple_signal*)(sig);
    printf("Signalling Signal %p\n",s);
    simple_signal_signal_and_exit(s);
    */
}

void ILetFunc(void *signal) {
/*
    printf("iLet on tile %u running on cpu %u with parameter %p\n", get_tile_id(), get_cpu_id(), signal);

    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    printf("Sending reply...\n");
    dispatch_claim_send_reply(&answer);
    */
}

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

    if (!myClaim) {
        die("Invade operation unsuccessful.");
    }


    printf("* Returned Claim:\n");
    agent_claim_print(myClaim);

    for(int i=0; i<10; i++) {
        printf("* Reinvading:\n");
        int ret = agent_claim_reinvade(myClaim);

        if (-1 == ret) {
            die("Reinvade operation unsuccessful.");
        }

        printf("* Returned Claim:\n");
        agent_claim_print(myClaim);
    }

    {
        // This block of code shows an example of how to infect an AgentOctoClaim.
        simple_signal sync;
        simple_signal_init(&sync, agent_claim_get_pecount(myClaim));

        for (int tile=0; tile < get_tile_count(); tile++) {
            int pes=agent_claim_get_pecount_tile_type(myClaim,  tile, 0);
            if (pes) { // Type = 0 ^= RISC
                proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(myClaim, tile, 0);
                printf("* Got Proxy Claim %p\n", pClaim);

                simple_ilet ILet[pes];
                for (int i = 0; i < pes; ++i) {
                    simple_ilet_init(&ILet[i], ILetFunc, &sync);
                }

                printf("Infecting %d Ilets on Tile %d\n", pes, tile);
                proxy_infect(pClaim, &ILet[0], pes);
            }
        }

        printf("Waiting on Signal %p...\n", &sync);
        simple_signal_wait(&sync);
        printf("All Signals received!\n");
    }

    return;

    printf("* Changing Constraints\n");
    agent_constr_set_quantity(myConstr, 2, 6, 0);   // min 2, max 6, type 0

    printf("* Reinvading:\n");
    int ret = agent_claim_reinvade_constraints(myClaim, myConstr);    // actually.. as we change myConstr (which is not copied), it is not necessary to use reinvade_constraints.. mhh..
                                                            // Actually, I believe the constraints are implicitly copied. A slot is not handling a pointer to a constraint, but
                                                            // a real Constraint object.

    if (-1 == ret) {
        die("Reinvade operation unsuccessful.");
    }

    printf("* Returned Claim:\n");
    agent_claim_print(myClaim);

    {
        // Yet another example of infecting an AgentOctoClaim.
        simple_signal sync;
        simple_signal_init(&sync, agent_claim_get_pecount(myClaim));

        for (int tile=0; tile < get_tile_count(); tile++) {
            int pes=agent_claim_get_pecount_tile_type(myClaim,  tile, 0);
            if (pes) { // Type = 0 ^= RISC
                proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(myClaim, tile, 0);
                printf("* Got Proxy Claim %p\n", pClaim);

                simple_ilet ILet[pes];
                for (int i = 0; i < pes; ++i) {
                    simple_ilet_init(&ILet[i], ILetFunc, &sync);
                }

                printf("Infecting %d Ilets on Tile %d\n", pes, tile);
                proxy_infect(pClaim, &ILet[0], pes);
            }
        }

        printf("Waiting on Signal %p...\n", &sync);
        simple_signal_wait(&sync);
        printf("All Signals received!\n");
    }

    printf("* Retreating:\n");
    agent_claim_retreat(myClaim);
    agent_constr_delete(myConstr);

    guest_shutdown();
}

