#include <stdio.h>
#include "octopos.h"


void linkertester() {
	printf("Hello World!\n");
	shutdown(0);
}

void proxy_infect_with_ilet(agentclaim_t claim, int pes, ilet_func ilet, void* param) {
    simple_ilet ilets[pes];
    for (int i = 0; i < pes; ++i) {
        // simple_ilet_init(&ilets[i], ilet_func, &param);
    }

    proxy_infect(claim, &ilets[0], pes);
}