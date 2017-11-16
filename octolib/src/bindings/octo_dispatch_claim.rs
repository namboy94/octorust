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
/// This fle is based on the IRTSS octo_dispatch_claim.h file

/// Dispatch claims wrap the dispatch information of claims.
///
/// Dispatch information is needed every time that an iLet is submitted to the
/// system.  Normally, dispatch information is given implicitly through the use
/// of a claim_t or proxy_claim_t. However, there are some instances where these
/// objects aren't available. in these cases it is possilble to extract the pure
/// dispatch information of claim in the form of a dispatch claim.
///
/// The only meaningful operation possible on a dispatch claim is infect (+ DMA
/// transfers which, from a control-flow perspective, are just rather fancy
/// infect calls)

use octo_types::*;
use octo_structs::*;

extern {

    /// Get a dispatch claim that describes the parent claim of the currently
    /// running ilet. The parent claim is the claim of the ilet executing the infect
    /// call that created the calling ilet.
    ///
    /// # Return Value
    ///
    /// dispatch claim describing the parent ilet
    pub fn get_parent_dispatch_claim() -> dispatch_claim_t;

    /// Get a dispatch claim that describes the currently active claim
    ///
    /// # Return Value
    ///
    /// dispatch claim describing the claim of the currently running ilet
    pub fn get_own_dispatch_claim() -> dispatch_claim_t;

    
    pub fn dispatch_claim_infect(claim: dispatch_claim_t, team: *mut simple_ilet, count: team_size_t);

    
    pub fn dispatch_claim_infect_single(claim: dispatch_claim_t, iLet: *mut simple_ilet);

    /// Infects the caller's own claim with a team of i-lets.///
    pub fn infect_self(team: *mut simple_ilet, count: team_size_t);

    /// Infects the caller's own claim with a single i-let.///
    pub fn infect_self_single(iLet: *mut simple_ilet);

    /// Setup a push DMA using a dispatch_claim_t instead of a proxy_claim_t
    /// If you don't have a proxy_claim_t of the destination, then a
    /// dispatch_claim_t also suffices to setup a push DMA transfer. E.g. this makes
    /// it possible to easily DMA back to your parent claim.
    /// Address format: On LEON, source and destination pointer have to be in the
    /// correct format:
    /// - local_src is a pointer to the local tlm:
    /// Address range according to memory map: 0x80000000-0x80800000 (local address
    /// format)
    /// - remote_dst is a pointer to the remote tlm:
    /// Address range according to memory map: Starting from 0x40000000 depending on
    /// TID (global address format)
    /// On LEON, accesses in program code to the local tlm ALWAYS have to be done via
    /// the local 0x80000000 addresses!
    ///
    /// # Arguments
    ///
    /// * `claim` - Destination claim on which dst_iLet will be submitted to
    /// * `local_src` - Source address in the local TLM (local address format)
    /// * `remote_dst` - Destination address in remote TLM (global address format)
    /// * `buf_size` - Size of the message
    /// * `src_iLet` - i-let that gets submitted at the source side of the transfer,
    ///                once the DMA operation is finished (or NULL if no i-let is to be executed)
    /// * `dst_iLet` - i-let that gets submitted at the destination side of the
    ///                transfer once the DMA operation is finished (or NULL if no i-let is to be
    ///                executed)
    ///
    /// # Return Value
    ///
    /// Returns 0 on success; On error -1 is returned: Checks consistency
    /// between remote_dst and TID in claim
    pub fn dispatch_claim_push_dma(claim: dispatch_claim_t, local_src: *const c_void, remote_dst: *mut c_void, buf_size: buf_size_t, src_iLet: *mut simple_ilet, dst_iLet: *mut simple_ilet) -> i32;

    /// Send an answer iLet to your direct parent claim.
    /// This is a shortcut for get_parent_dispatch_claim() followed by a
    /// dispatch_claim_infect with one iLet.
    ///
    /// # Arguments
    ///
    /// * `iLet` - gets sent to your parent claim
    pub fn dispatch_claim_send_reply(iLet: *mut simple_ilet);

    /// Get tile id of given dispatch claim
    ///
    /// # Arguments
    ///
    /// * `dc` - dispatch claim to get tile ID from
    ///
    /// # Return Value
    ///
    /// tid of dispatch claim
    pub fn dispatch_claim_get_tid(dc: dispatch_claim_t) -> tile_id_t;

}
