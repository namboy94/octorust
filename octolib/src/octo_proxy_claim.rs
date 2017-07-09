/// octo_proxy_claim.h
/// 
/// Functions related to proxy_claim objects for potentially distant claims

use octo_types::*;
use libc::c_char;

/// @struct proxy_claim
/// ProxyClaims are system data structures: Hence, only a Pointer is needed.
pub type proxy_claim_t = *c_void;


/// struct invade_future
/// Opaque type for invade_future
///
/// This type is used to allocate invade_future "objects" with the right size.
#[repr(C)]
pub struct invade_future {
    pub padding: [c_char; 64usize],
}
pub type invade_future_t = invade_future;

/// struct reinvade_future
/// Opaque type for reinvade_future
///
/// This type is used to allocate reinvade_future "objects" with the right size.
#[repr(C)]
pub struct reinvade_future {
    pub padding: [c_char; 64usize],
}
pub type reinvade_future_t = reinvade_future;

#[repr(C)]
pub struct retreat_future {
    pub padding: [c_char; 64usize],
}
pub type retreat_future_t = retreat_future;

/// Get the proxy claim returned from a previous proxy_invade
///
/// Use this to get the return value from a previous proxy_invade call. On
/// success this is a pointer to a proxy claim object or 0 on failure. If the
/// remote_invade hasn't finished, this call will block until it has.
///
/// \param fut pointer to the future provided to proxy_invade
/// \return if successful, returns a reference to a proxy claim object. Returns
/// 0 if the invade request could not be satisfied
proxy_claim_t invade_future_force(invade_future_t* fut);


/// Get return value for reinvade
/// 
/// Use this to get return value from previously triggered reinvade. Blocks
/// until reinvade has finished.
///
/// \param fut Pointer to future that was provided to proxy_reinvade
/// \return Returns 0 on success, -1 on error
int reinvade_future_force(reinvade_future_t* fut);


/// Get return value for retreat
/// 
/// Use this to get return value from previously triggered retreat. Blocks
/// until retreat has finished.
///
/// \param fut Pointer to future that was provided to proxy_retreat
/// \return Returns 0 on success, -1 on error
int retreat_future_force(retreat_future_t* fut);


/// Invade creating a new claim; claim can potentially be located on
/// another tile
/// 
/// This call primarily serves to invade other tiles in the system. This call is
/// serviced asynchronously: Instead of blocking until the potentially remote
/// resource allocaton has finished, it uses a future object that can be used to
/// access the return value of the call 
///
/// \param fut future object passed by the application. Can be used to get the
/// return value of proxy_invade
/// \param tile_id tile id of the tile you want to invade
/// \param quantity number of PEs to acquire
/// \return 0 on success, -1 on error.
int proxy_invade(tile_id_t tile_id, invade_future_t* fut, pe_quantity_t quantity);


/// Change resource allocation on an already existing claim
/// 
/// This call changes the resource allocation of a potentially distant claim.
/// This call is, like proxy_invade, also asynchronous, exposing its return
/// value as a future.
///
/// \param claim Existing proxy claim that shall be modified
/// \param fut Future to query success and completion of operation
/// \param deltaPE change claim size by deltaPE
void proxy_reinvade(proxy_claim_t claim, reinvade_future_t* fut, pe_quantity_delta_t deltaPE);


/// Change resource allocation on an already existing claim
///
/// This basically is the same operation as proxy_reinvade, but it is executed
/// synchronously and its result is returned directly.
///
/// \param claim Existing proxy claim that shall be modified
/// \param deltaPE change claim size by deltaPE
/// \return 0 on success, -1 on error
int proxy_reinvade_sync(proxy_claim_t claim, pe_quantity_delta_t deltaPE);


/// This completely dissolves the claim and frees its resources
///
/// proxy_retreat completely destroys the claim that is associated with this
/// proxy_claim_t object. This means all cores are freed and afterwards can be
/// invaded by other claims. This call also deletes the proxy_claim_t object
/// itself. Using this reference after the retreat call leads to undefined
/// behaviour.
///
/// \param claim Existing proxy_claim that shall be retreated from
/// \param fut Future object to query return value and synchronise on completion
/// of the operation
void proxy_retreat(proxy_claim_t claim, retreat_future_t* fut);


/// This completely dissolves the claim and frees its resources
///
/// This basically is the same operation as proxy_retreat. It completely
/// destroys the claim and the proxy_claim_t, but is executed synchronously and
/// its result is returned directly.
///
/// \param claim Existing proxy_claim that shall be retreated from
/// \return 0 on success, -1 on error
int proxy_retreat_sync(proxy_claim_t claim);
 
/// Executing a team of iLets on a previously created proxy claim
///
/// \param claim claim of the application the iLet belongs to. The claim defines
/// to which tile and set of cores the ilet is forwarded.
/// \param team Array of iLets forming the team
/// \param count number of iLets forming the team
void proxy_infect(proxy_claim_t claim, simple_ilet team[], team_size_t count);


/// Setup a push DMA operation
/// 
/// iNoC DMAs are push DMAs, so you should submit DMA transfers on the tile that
/// holds the local_src buffer. The claim parameter has to match remote_dst,
/// meaning that the claim wrapped by the proxy_claim_t claim has to reside on
/// the same tile that remote_dst does.
/// 
/// Address format: On LEON, source and destination pointer have to be in the
/// correct format:
/// - \b local_src is a pointer to the local tlm:
/// Address range according to memory map: 0x80000000-0x80800000 (local address
/// format)
/// - \b remote_dst is a pointer to the remote tlm:
/// Address range according to memory map: Starting from 0x40000000 depending on
/// TID (global address format)
///
/// On LEON, accesses in program code to the local tlm ALWAYS have to be done via
/// the local 0x80000000 addresses!
///
/// \param claim Destination claim on which dst_iLet will be submitted to
/// \param local_src Source address in the local TLM (local address format)
/// \param remote_dst Destination address in remote TLM (global address format)
/// \param buf_size Size of the message
/// \param src_iLet i-let that gets submitted at the source side of the transfer,
/// once the DMA operation is finished (or NULL if no i-let is to be executed)
/// \param dst_iLet i-let that gets submitted at the destination side of the
/// transfer once the DMA operation is finished (or NULL if no i-let is to be
/// executed)
/// \return Returns 0 on success; On error -1 is returned: Checks consistency
/// between \b remote_dst and TID in claim
int proxy_push_dma(proxy_claim_t claim, const void* local_src, void* remote_dst, buf_size_t buf_size,
		simple_ilet* src_iLet, simple_ilet* dst_iLet);


/// Get TileID of proxy claim
tile_id_t proxy_get_tid(proxy_claim_t claim);


/// Extract dispatch information from a proxy claim
///
/// \param claim Proxy claim to extract dispatch information from
/// \return dispatch claim representing dispatch inforamtion of proxy claim
dispatch_claim_t proxy_get_dispatch_info(proxy_claim_t claim);



/// This API function is unstable, do not use it
///
uintptr_t proxy_get_available_mask(void);



/// This API function is unstable, do not use it
///
int proxy_reinvade_mask(uintptr_t mask);