/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_claim.h file
///
/// Functions operating on a tile-local claim

use octo_types::*;
use octo_structs::*;

extern {

    /// Allocate a new tile-local claim object
    /// This allocates a strictly tile-local claim object (note distinction between claim_t and proxy_claim_t).
    ///
    /// # Return Value
    ///
    /// tile-local claim descriptor
    pub fn claim_construct() -> claim_t;

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
    pub fn invade_simple(claim: claim_t, quantity: u32) -> i32;

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
    ///    - There are still i-lets in the CiC FIFOs waiting to be executed.
    ///    - The claim's heap cannot be shrunk.
    pub fn retreat(claim: claim_t, quantity: u32) -> i32;

    /// Does a tile-local infect on a tile-local claim
    pub fn infect(claim: claim_t, team: *mut simple_ilet, team_size: u32) -> i32;

    /// Infects the current claim with a single ilet.
    pub fn infect_simple(ilet: *mut simple_ilet) -> i32;

    /// Returns the current claim the calling iLet runs on.
    /// This function yields the local claim descriptor that belongs to the current
    /// iLet. This also works if the infect operation happend through a ProxyClaim.
    /// This is useful to access claim-local data in a program.
    ///
    /// # Return Value
    ///
    /// current tile-local claim descriptor
    pub fn get_claim() -> claim_t;

    /// Set claim-local data pointer
    /// Claim-local data can be used to keep track of data between iLets
    ///
    /// # Arguments
    ///
    /// * `arg` - pointer to the data that shall be claim-local
    pub fn claim_set_local_data(arg: *mut c_void);

    /// Get claim-local data pointer
    ///
    /// # Return Value
    ///
    /// pointer to the own claim's claim-local data, or NULL if no
    /// claim-local data has been set
    pub fn claim_get_local_data() -> *mut c_void;

    /// Query the number of allocated cores
    ///
    /// # Return Value
    ///
    /// Numer of cores the own claim has
    pub fn claim_get_size() -> u32;

}
