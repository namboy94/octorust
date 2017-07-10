#ifndef _OCTO_TCPA_H_
#define _OCTO_TCPA_H_

#include "octo_tcpa_types.h"
#include "octo_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* tcpa_proxy_claim_t;
typedef void* tcpa_infect_response_t;
typedef void* tcpa_output_transfer_confirm_t;

struct tcpa_invade_future {
	char padding[TCPA_INVADE_FUTURE_SIZE];
}__attribute__((aligned(TCPA_INVADE_FUTURE_ALIGNMENT)));
typedef struct tcpa_invade_future tcpa_invade_future_t;

struct tcpa_infect_future {
	char padding[TCPA_HARRIS_INFECT_FUTURE_SIZE];
}__attribute__((aligned(TCPA_HARRIS_INFECT_FUTURE_ALIGNMENT)));
typedef struct tcpa_infect_future tcpa_infect_future_t;

struct tcpa_get_output_future {
	char padding[TCPA_GET_OUTPUT_FUTURE_SIZE];
}__attribute__((aligned(TCPA_GET_OUTPUT_FUTURE_ALIGNMENT)));
typedef struct tcpa_get_output_future tcpa_get_output_future_t;

int tcpa_proxy_invade(tcpa_invade_future_t* fut, pe_quantity_t quantity0, pe_quantity_t quantity1);

tcpa_proxy_claim_t tcpa_invade_future_force(tcpa_invade_future_t* fut);

int tcpa_proxy_retreat(tcpa_proxy_claim_t claim);

int tcpa_proxy_infect(tcpa_proxy_claim_t claim, tcpa_infect_future_t* fut, const void* input, int i_size, const void* binary, int b_size); 

int tcpa_proxy_reinfect(tcpa_proxy_claim_t claim, tcpa_infect_future_t* fut, const void* input, int i_size); 

tcpa_infect_response_t tcpa_infect_future_force(tcpa_infect_future_t* fut);

int tcpa_proxy_get_id(tcpa_proxy_claim_t claim);

int tcpa_proxy_get_claim_size(tcpa_proxy_claim_t claim);

int tcpa_get_output_size(tcpa_infect_response_t hResp);

int tcpa_get_output_stream(tcpa_proxy_claim_t claim, tcpa_get_output_future_t* fut, const void* input); 

tcpa_output_transfer_confirm_t tcpa_get_output_future_force(tcpa_get_output_future_t* fut);

int tcpa_check_output_transmission(tcpa_output_transfer_confirm_t transferConf); 

#ifdef __cplusplus
}
#endif


#endif
