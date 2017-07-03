#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "octopos.h"
#include "octo_agent.h"

const int parnum=2;
int globCounter[parnum];

void StressILet(void* parm) {

    constraints_t stressConstraints;
    agentclaim_t stressClaim;

    stressConstraints= agent_constr_create();
    agent_constr_set_quantity(stressConstraints, 1, 1+rand()%15, 0);
    agent_constr_set_tile_shareable(stressConstraints, 1);
    agent_constr_set_appnumber(stressConstraints, (int)parm);
    stressClaim = agent_claim_invade(NULL, stressConstraints); // creates an agentclaim_t object!

    if (!stressClaim) {
        fprintf(stderr, "Invade operation unsuccessful.");
        abort();
    }

    globCounter[(int)parm]++;
    if(globCounter[(int)parm] % 1000 == 0)
     printf("* StressiLet %d got Claim of Size %d [%d]\n", (int)parm, agent_claim_get_pecount(stressClaim), globCounter[(int)parm]);

//     agent_claim_print(stressClaim);


    agent_claim_retreat(stressClaim); // deletes the claim object!
    agent_constr_delete(stressConstraints);

    simple_ilet ILet;
    simple_ilet_init(&ILet, StressILet, parm);
    infect_self_single(&ILet);
}


extern "C" void main_ilet(claim_t claim) {
    printf("main ilet\n");

    agentclaim_t initialClaim = agent_claim_get_initial(claim);

    printf("** Invading %d Claims for Stresstest:\n", parnum);

    constraints_t myConstr[parnum];
    agentclaim_t myClaim[parnum];

    for (int i=0; i<parnum; i++) {
        globCounter[i]=0;
        myConstr[i] = agent_constr_create();
        agent_constr_set_quantity(myConstr[i], 1, 1, 0);   // min 2, max 5, type 0
        agent_constr_set_tile_shareable(myConstr[i], 1);
        agent_constr_set_appnumber(myConstr[i], i);
        myClaim[i] = agent_claim_invade(NULL, myConstr[i]);

        if (!myClaim[i]) {
            fprintf(stderr, "Invade operation unsuccessful.");
            abort();
        }

        printf("* Returned Claim %d of Size %d\n", i, agent_claim_get_pecount(myClaim[i]));
    }

    for (int i=0; i<parnum; i++) {
        for (int tile=0; tile < get_tile_count(); tile++) {
            if (int pes=agent_claim_get_pecount_tile_type(myClaim[i],  tile, 0)) {	// Type = 0 ^= RISC
                proxy_claim_t pClaim = agent_claim_get_proxyclaim_tile_type(myClaim[i], tile, 0);
                printf("* Got Proxy Claim %p\n", pClaim);

                simple_ilet ILet[pes];
                for (int iletnr = 0; iletnr < pes; ++iletnr) {
                    simple_ilet_init(&ILet[iletnr], StressILet, (void*)i); // parms);
                }

                printf("Infecting %d Ilets on Tile %d\n", pes, tile);
                proxy_infect(pClaim, &ILet[0], pes);
            }
        }
    }


    return;
}
