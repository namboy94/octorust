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
