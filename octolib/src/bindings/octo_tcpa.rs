/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_tcpa.h file

use octo_structs::*;
use octo_types::*;

extern {
    #[link_name="tcpa_proxy_invade"]
    fn __tcpa_proxy_invade(fut: *mut tcpa_invade_future_t, quantity0: pe_quantity_t, quantity1: pe_quantity_t) -> i32;

    #[link_name="tcpa_invade_future_force"]
    fn __tcpa_invade_future_force(fut: *mut tcpa_invade_future_t) -> tcpa_proxy_claim_t;

    #[link_name="tcpa_proxy_retreat"]
    fn __tcpa_proxy_retreat(claim: tcpa_proxy_claim_t) -> i32;

    #[link_name="tcpa_proxy_infect"]
    fn __tcpa_proxy_infect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t, input: *const c_void, i_size: i32, binary: *const c_void, b_size: i32) -> i32;

    #[link_name="tcpa_proxy_reinfect"]
    fn __tcpa_proxy_reinfect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t, input: *const c_void, i_size: i32) -> i32;

    #[link_name="tcpa_infect_future_force"]
    fn __tcpa_infect_future_force(fut: *mut tcpa_infect_future_t) -> tcpa_infect_response_t;

    #[link_name="tcpa_proxy_get_id"]
    fn __tcpa_proxy_get_id(claim: tcpa_proxy_claim_t) -> i32;

    #[link_name="tcpa_proxy_get_claim_size"]
    fn __tcpa_proxy_get_claim_size(claim: tcpa_proxy_claim_t) -> i32;

    #[link_name="tcpa_get_output_size"]
    fn __tcpa_get_output_size(hResp: tcpa_infect_response_t) -> i32;

    #[link_name="tcpa_get_output_stream"]
    fn __tcpa_get_output_stream(claim: tcpa_proxy_claim_t, fut: *mut tcpa_get_output_future_t, input: *const c_void) -> i32;

    #[link_name="tcpa_get_output_future_force"]
    fn __tcpa_get_output_future_force(fut: *mut tcpa_get_output_future_t) -> tcpa_output_transfer_confirm_t;

    #[link_name="tcpa_check_output_transmission"]
    fn __tcpa_check_output_transmission(transferConf: tcpa_output_transfer_confirm_t) -> i32;

}

pub fn tcpa_proxy_invade(fut: *mut tcpa_invade_future_t, quantity0: pe_quantity_t, quantity1: pe_quantity_t) -> i32 {
    unsafe {
        __tcpa_proxy_invade(fut, quantity0, quantity1)
    }
}

pub fn tcpa_invade_future_force(fut: *mut tcpa_invade_future_t) -> tcpa_proxy_claim_t {
    unsafe {
        __tcpa_invade_future_force(fut)
    }
}

pub fn tcpa_proxy_retreat(claim: tcpa_proxy_claim_t) -> i32 {
    unsafe {
        __tcpa_proxy_retreat(claim)
    }
}

pub fn tcpa_proxy_infect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t, input: *const c_void, i_size: i32, binary: *const c_void, b_size: i32) -> i32 {
    unsafe {
        __tcpa_proxy_infect(claim, fut, input, i_size, binary, b_size)
    }
}

pub fn tcpa_proxy_reinfect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t, input: *const c_void, i_size: i32) -> i32 {
    unsafe {
        __tcpa_proxy_reinfect(claim, fut, input, i_size)
    }
}

pub fn tcpa_infect_future_force(fut: *mut tcpa_infect_future_t) -> tcpa_infect_response_t {
    unsafe {
        __tcpa_infect_future_force(fut)
    }
}

pub fn tcpa_proxy_get_id(claim: tcpa_proxy_claim_t) -> i32 {
    unsafe {
        __tcpa_proxy_get_id(claim)
    }
}

pub fn tcpa_proxy_get_claim_size(claim: tcpa_proxy_claim_t) -> i32 {
    unsafe {
        __tcpa_proxy_get_claim_size(claim)
    }
}

pub fn tcpa_get_output_size(hResp: tcpa_infect_response_t) -> i32 {
    unsafe {
        __tcpa_get_output_size(hResp)
    }
}

pub fn tcpa_get_output_stream(claim: tcpa_proxy_claim_t, fut: *mut tcpa_get_output_future_t, input: *const c_void) -> i32 {
    unsafe {
        __tcpa_get_output_stream(claim, fut, input)
    }
}

pub fn tcpa_get_output_future_force(fut: *mut tcpa_get_output_future_t) -> tcpa_output_transfer_confirm_t {
    unsafe {
        __tcpa_get_output_future_force(fut)
    }
}

pub fn tcpa_check_output_transmission(transferConf: tcpa_output_transfer_confirm_t) -> i32 {
    unsafe {
        __tcpa_check_output_transmission(transferConf)
    }
}

