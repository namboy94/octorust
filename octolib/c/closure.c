#include <octopos.h>

typedef void (*rust_closure_function) (void*, void*);

void c_closure_infect(
    int pes,
    proxy_claim_t claim,
    rust_closure_function ilet,
    void* closure_data,
    void* params
) {
/*
    simple_ilet ilets[pes];
    for (int i = 0; i < pes; i++) {
        dual_ilet_init(simple_ilet[i], ilet, closure_data, params);
    }
    proxy_infect(claim, ilets[0], pes);
    */
}

