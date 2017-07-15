use octo_types::*;

pub type tcpa_proxy_claim_t = *mut c_void;
pub type tcpa_infect_response_t = *mut c_void;
pub type tcpa_output_transfer_confirm_t = *mut c_void;

#[repr(C)]
pub struct tcpa_invade_future {
    pub padding: [c_char; TCPA_INVADE_FUTURE_SIZE],
}
pub type tcpa_invade_future_t = tcpa_invade_future;

#[repr(C)]
pub struct tcpa_infect_future {
    pub padding: [c_char; TCPA_INFECT_FUTURE_SIZE],
}
pub type tcpa_infect_future_t = tcpa_infect_future;

#[repr(C)]
pub struct tcpa_get_output_future {
    pub padding: [c_char; TCPA_GET_OUTPUT_FUTURE_SIZE],
}
pub type tcpa_get_output_future_t = tcpa_get_output_future;