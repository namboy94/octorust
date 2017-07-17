/// Functions operating on a tile-local claim

use octo_types::*;
use octo_structs::*;

extern {
    #[link_name="claim_construct"]
    fn __claim_construct() -> claim_t;

    #[link_name="invade_simple"]
    fn __invade_simple(claim: claim_t, quantity: u32) -> i32;

    #[link_name="retreat"]
    fn __retreat(claim: claim_t, quantity: u32) -> i32;

    #[link_name="infect"]
    fn __infect(claim: claim_t, team: *mut simple_ilet, team_size: u32) -> i32;

    #[link_name="infect_simple"]
    fn __infect_simple(ilet: *mut simple_ilet) -> i32;

    #[link_name="get_claim"]
    fn __get_claim() -> claim_t;

    #[link_name="claim_set_local_data"]
    fn __claim_set_local_data(arg: *mut c_void);

    #[link_name="claim_get_local_data"]
    fn __claim_get_local_data() -> *mut c_void;

    #[link_name="claim_get_size"]
    fn __claim_get_size() -> u32;

}

/// Allocate a new tile-local claim object
/// This allocates a strictly tile-local claim object (note distinction between
/// claim_t and proxy_claim_t).
///
/// # Return Value
///
/// tile-local claim descriptor
pub fn claim_construct() -> claim_t {
    unsafe {
        __claim_construct()
    }
}

/// Do a tile-local invade on a strictly tile-local claim descriptor
///
/// # Arguments
///
/// * `claim` - tile-local claim descriptor
/// * `quantity` - number of PEs to be invaded
///
/// # Return Value
///
/// 0 if successful, -1 if failed
pub fn invade_simple(claim: claim_t, quantity: u32) -> i32 {
    unsafe {
        __invade_simple(claim, quantity)
    }
}

/// Do a tile-local retreat on a strictly tile-local claim descriptor
///
/// # Arguments
///
/// * `claim` - tile-local claim descriptor
/// * `quantity` - number of PEs to be retreated from
///
/// # Return Value
///
/// 0 on success, -1 on error
///
/// # Note
///
///  Possible reasons for retreat() to fail:
///       - There are still i-lets in the CiC FIFOs waiting to be executed.
///       - The claim's heap cannot be shrunk.
pub fn retreat(claim: claim_t, quantity: u32) -> i32 {
    unsafe {
        __retreat(claim, quantity)
    }
}

/// brief Does a tile-local infect on a tile-local claim
pub fn infect(claim: claim_t, team: *mut simple_ilet, team_size: u32) -> i32 {
    unsafe {
        __infect(claim, team, team_size)
    }
}

/// Infects the current claim with a single ilet.
pub fn infect_simple(ilet: *mut simple_ilet) -> i32 {
    unsafe {
        __infect_simple(ilet)
    }
}

/// Returns the current claim the calling iLet runs on.
/// This function yields the local claim descriptor that belongs to the current
/// iLet. This also works if the infect operation happend through a ProxyClaim.
/// This is useful to access claim-local data in a program.
///
/// # Return Value
///
/// current tile-local claim descriptor
pub fn get_claim() -> claim_t {
    unsafe {
        __get_claim()
    }
}

/// Set claim-local data pointer
/// Claim-local data can be used to keep track of data between iLets
///
/// # Arguments
///
/// * `arg` - pointer to the data that shall be claim-local
pub fn claim_set_local_data(arg: *mut c_void) {
    unsafe {
        __claim_set_local_data(arg)
    }
}

/// Get claim-local data pointer
///
/// # Return Value
///
/// pointer to the own claim's claim-local data, or \c NULL if no
/// claim-local data has been set
pub fn claim_get_local_data() -> *mut c_void {
    unsafe {
        __claim_get_local_data()
    }
}

/// Query the number of allocated cores
///
/// # Return Value
///
/// Numer of cores the own claim has
pub fn claim_get_size() -> u32 {
    unsafe {
        __claim_get_size()
    }
}
