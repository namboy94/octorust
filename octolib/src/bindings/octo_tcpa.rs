/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_tcpa.h file

use octo_structs::*;
use octo_types::*;

extern {
    
    pub fn tcpa_proxy_invade(fut: *mut tcpa_invade_future_t, quantity0: pe_quantity_t, quantity1: pe_quantity_t)
        -> i32;

    pub fn tcpa_invade_future_force(fut: *mut tcpa_invade_future_t) -> tcpa_proxy_claim_t;

    pub fn tcpa_proxy_retreat(claim: tcpa_proxy_claim_t) -> i32;

    pub fn tcpa_proxy_infect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t,
                             input: *const c_void, i_size: i32, binary: *const c_void, b_size: i32) -> i32;

    pub fn tcpa_proxy_reinfect(claim: tcpa_proxy_claim_t, fut: *mut tcpa_infect_future_t,
                               input: *const c_void, i_size: i32) -> i32;

    pub fn tcpa_infect_future_force(fut: *mut tcpa_infect_future_t) -> tcpa_infect_response_t;

    pub fn tcpa_proxy_get_id(claim: tcpa_proxy_claim_t) -> i32;

    pub fn tcpa_proxy_get_claim_size(claim: tcpa_proxy_claim_t) -> i32;

    pub fn tcpa_get_output_size(hResp: tcpa_infect_response_t) -> i32;

    pub fn tcpa_get_output_stream(claim: tcpa_proxy_claim_t, fut: *mut tcpa_get_output_future_t, input: *const c_void)
        -> i32;

    pub fn tcpa_get_output_future_force(fut: *mut tcpa_get_output_future_t) -> tcpa_output_transfer_confirm_t;

    pub fn tcpa_check_output_transmission(transferConf: tcpa_output_transfer_confirm_t) -> i32;

}
