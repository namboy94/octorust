use octo_types::*;
use octo_structs::*;



extern {
    #[link_name="invade_future_force"]
    fn __invade_future_force(fut: *mut invade_future_t) -> proxy_claim_t;

    #[link_name="reinvade_future_force"]
    fn __reinvade_future_force(fut: *mut reinvade_future_t) -> i32;

    #[link_name="retreat_future_force"]
    fn __retreat_future_force(fut: *mut retreat_future_t) -> i32;

    #[link_name="proxy_invade"]
    fn __proxy_invade(tile_id: tile_id_t, fut: *mut invade_future_t, quantity: pe_quantity_t) -> i32;

    #[link_name="proxy_reinvade"]
    fn __proxy_reinvade(claim: proxy_claim_t, fut: *mut reinvade_future_t, deltaPE: pe_quantity_delta_t);

    #[link_name="proxy_reinvade_sync"]
    fn __proxy_reinvade_sync(claim: proxy_claim_t, deltaPE: pe_quantity_delta_t) -> i32;

    #[link_name="proxy_retreat"]
    fn __proxy_retreat(claim: proxy_claim_t, fut: *mut retreat_future_t);

    #[link_name="proxy_retreat_sync"]
    fn __proxy_retreat_sync(claim: proxy_claim_t) -> i32;

    #[link_name="proxy_infect"]
    fn __proxy_infect(claim: proxy_claim_t, team: *mut simple_ilet, count: team_size_t);

    #[link_name="proxy_push_dma"]
    fn __proxy_push_dma(claim: proxy_claim_t, local_src: *const c_void, remote_dst: *mut c_void, buf_size: buf_size_t, src_iLet: *mut simple_ilet, dst_iLet: *mut simple_ilet) -> i32;

    #[link_name="proxy_get_tid"]
    fn __proxy_get_tid(claim: proxy_claim_t) -> tile_id_t;

    #[link_name="proxy_get_dispatch_info"]
    fn __proxy_get_dispatch_info(claim: proxy_claim_t) -> dispatch_claim_t;

    #[link_name="proxy_get_available_mask"]
    fn __proxy_get_available_mask() -> u64;

    #[link_name="proxy_reinvade_mask"]
    fn __proxy_reinvade_mask(mask: u64) -> i32;

}

/// Get the proxy claim returned from a previous proxy_invade
/// Use this to get the return value from a previous proxy_invade call. On
/// success this is a pointer to a proxy claim object or 0 on failure. If the
/// remote_invade hasn't finished, this call will block until it has.
///
/// # Arguments
///
/// * `fut` - pointer to the future provided to proxy_invade
///
/// # Return Value
///
/// if successful, returns a reference to a proxy claim object. Returns
///
/// 0 if the invade request could not be satisfied
pub fn invade_future_force(fut: *mut invade_future_t) -> proxy_claim_t {
    unsafe {
        __invade_future_force(fut)
    }
}

/// Get return value for reinvade
/// Use this to get return value from previously triggered reinvade. Blocks
/// until reinvade has finished.
///
/// # Arguments
///
/// * `fut` - Pointer to future that was provided to proxy_reinvade
///
/// # Return Value
///
/// Returns 0 on success, -1 on error
pub fn reinvade_future_force(fut: *mut reinvade_future_t) -> i32 {
    unsafe {
        __reinvade_future_force(fut)
    }
}

/// Get return value for retreat
/// Use this to get return value from previously triggered retreat. Blocks
/// until retreat has finished.
///
/// # Arguments
///
/// * `fut` - Pointer to future that was provided to proxy_retreat
///
/// # Return Value
///
/// Returns 0 on success, -1 on error
pub fn retreat_future_force(fut: *mut retreat_future_t) -> i32 {
    unsafe {
        __retreat_future_force(fut)
    }
}

/// Invade creating a new claim; claim can potentially be located on
/// another tile
/// This call primarily serves to invade other tiles in the system. This call is
/// serviced asynchronously: Instead of blocking until the potentially remote
/// resource allocaton has finished, it uses a future object that can be used to
/// access the return value of the call
///
/// # Arguments
///
/// * `fut` - future object passed by the application. Can be used to get the
///           return value of proxy_invade
/// * `tile_id` - tile id of the tile you want to invade
/// * `quantity` - number of PEs to acquire
///
/// # Return Value
///
/// 0 on success, -1 on error.
pub fn proxy_invade(tile_id: tile_id_t, fut: *mut invade_future_t, quantity: pe_quantity_t) -> i32 {
    unsafe {
        __proxy_invade(tile_id, fut, quantity)
    }
}

/// Change resource allocation on an already existing claim
/// This call changes the resource allocation of a potentially distant claim.
/// This call is, like proxy_invade, also asynchronous, exposing its return
/// value as a future.
///
/// # Arguments
///
/// * `claim` - Existing proxy claim that shall be modified
/// * `fut` - Future to query success and completion of operation
/// * `deltaPE` - change claim size by deltaPE
pub fn proxy_reinvade(claim: proxy_claim_t, fut: *mut reinvade_future_t, deltaPE: pe_quantity_delta_t) {
    unsafe {
        __proxy_reinvade(claim, fut, deltaPE)
    }
}

/// Change resource allocation on an already existing claim
/// This basically is the same operation as proxy_reinvade, but it is executed
/// synchronously and its result is returned directly.
///
/// # Arguments
///
/// * `claim` - Existing proxy claim that shall be modified
/// * `deltaPE` - change claim size by deltaPE
///
/// # Return Value
///
/// 0 on success, -1 on error
pub fn proxy_reinvade_sync(claim: proxy_claim_t, deltaPE: pe_quantity_delta_t) -> i32 {
    unsafe {
        __proxy_reinvade_sync(claim, deltaPE)
    }
}

/// This completely dissolves the claim and frees its resources
/// proxy_retreat completely destroys the claim that is associated with this
/// proxy_claim_t object. This means all cores are freed and afterwards can be
/// invaded by other claims. This call also deletes the proxy_claim_t object
/// itself. Using this reference after the retreat call leads to undefined
/// behaviour.
///
/// # Arguments
///
/// * `claim` - Existing proxy_claim that shall be retreated from
/// * `fut` - Future object to query return value and synchronise on completion
///           of the operation
pub fn proxy_retreat(claim: proxy_claim_t, fut: *mut retreat_future_t) {
    unsafe {
        __proxy_retreat(claim, fut)
    }
}

/// This completely dissolves the claim and frees its resources
/// This basically is the same operation as proxy_retreat. It completely
/// destroys the claim and the proxy_claim_t, but is executed synchronously and
/// its result is returned directly.
///
/// # Arguments
///
/// * `claim` - Existing proxy_claim that shall be retreated from
///
/// # Return Value
///
/// 0 on success, -1 on error
pub fn proxy_retreat_sync(claim: proxy_claim_t) -> i32 {
    unsafe {
        __proxy_retreat_sync(claim)
    }
}

/// Executing a team of iLets on a previously created proxy claim
///
/// # Arguments
///
/// * `claim` - claim of the application the iLet belongs to. The claim defines
///             to which tile and set of cores the ilet is forwarded.
/// * `team` - Array of iLets forming the team
/// * `count` - number of iLets forming the team
pub fn proxy_infect(claim: proxy_claim_t, team: *mut simple_ilet, count: team_size_t) {
    unsafe {
        __proxy_infect(claim, team, count)
    }
}

/// Setup a push DMA operation
/// iNoC DMAs are push DMAs, so you should submit DMA transfers on the tile that
/// holds the local_src buffer. The claim parameter has to match remote_dst,
/// meaning that the claim wrapped by the proxy_claim_t claim has to reside on
/// the same tile that remote_dst does.
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
/// # Return Value
///
/// Returns 0 on success; On error -1 is returned: Checks consistency
/// between remote_dst and TID in claim
pub fn proxy_push_dma(claim: proxy_claim_t, local_src: *const c_void, remote_dst: *mut c_void, buf_size: buf_size_t, src_iLet: *mut simple_ilet, dst_iLet: *mut simple_ilet) -> i32 {
    unsafe {
        __proxy_push_dma(claim, local_src, remote_dst, buf_size, src_iLet, dst_iLet)
    }
}

/// Get TileID of proxy claim
pub fn proxy_get_tid(claim: proxy_claim_t) -> tile_id_t {
    unsafe {
        __proxy_get_tid(claim)
    }
}

/// Extract dispatch information from a proxy claim
///
/// # Arguments
///
/// * `claim` - Proxy claim to extract dispatch information from
///
/// # Return Value
///
/// dispatch claim representing dispatch inforamtion of proxy claim
pub fn proxy_get_dispatch_info(claim: proxy_claim_t) -> dispatch_claim_t {
    unsafe {
        __proxy_get_dispatch_info(claim)
    }
}

/// This API function is unstable, do not use it
pub fn proxy_get_available_mask() -> u64 {
    unsafe {
        __proxy_get_available_mask()
    }
}

/// This API function is unstable, do not use it
pub fn proxy_reinvade_mask(mask: u64) -> i32 {
    unsafe {
        __proxy_reinvade_mask(mask)
    }
}

