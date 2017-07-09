#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "octopos.h"
#include "octo_agent.h"

const int parnum=2;
int globCounter[2];


void main_ilet(claim_t claim) {

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
    return;
}