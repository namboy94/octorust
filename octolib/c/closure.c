#include <octopos.h>

typedef void (*rust_closure_function) (void*, void*);

void closure_infect(
    agentclaim_t claim,
    rust_closure_function ilet,
    void* closure_data,
    void* params
) {
    for (int tile = 0; tile < get_tile_count(); tile++) {
        int pes=agent_claim_get_pecount_tile_type(claim, tile, 0);
        if (pes) { // Type = 0 ^= RISC
            proxy_claim_t proxy_claim = agent_claim_get_proxyclaim_tile_type(claim, tile, 0);

            simple_ilet ilets[pes];
            for (int i = 0; i < pes; ++i) {
                dual_ilet_init(&ilets[i], ilet, closure_data, params);
            }

            proxy_infect(proxy_claim, &ilets[0], pes);
        }
    }
}

