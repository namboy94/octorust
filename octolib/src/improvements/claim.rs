/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Import printf for printing methods
extern { fn printf(s: *const u8, ...); }

// Imports
use core::ptr;
use improvements::constraints::Constraints;
use octo_agent::{agent_claim_invade, agent_claim_retreat, agent_claim_reinvade,
                 agent_claim_reinvade_constraints, agent_claim_get_pecount_tile_type,
                 agent_claim_get_proxyclaim_tile_type, agent_claim_get_pecount};
use octo_types::{agentclaim_t, ilet_func};
use octo_structs::{simple_ilet};
use octo_ilet::{simple_ilet_init};
use octo_proxy_claim::{proxy_infect};
use octo_tile::{get_tile_count};

/// The AgentClaim struct wraps around an agentclaim_t object to offer
/// a simplified interface to methods associated with an agent claim, implementing
/// an invade, infect, reinvade and implicit retreat methods.
///
/// # Members
///
/// * `claim` - The agentclaim_t struct that this struct wraps around
pub struct AgentClaim {
    claim: agentclaim_t,
    verbose: bool
}

/// Implementation of the AgentClaim struct
impl AgentClaim {

    /// Constructor for the AgentClaim struct. Automatically invades
    /// using the specified constraints
    ///
    /// # Arguments
    ///
    /// * `constraints` - The constraints of this claim
    pub fn new(constraints: Constraints) -> AgentClaim {
        let constraints = constraints.to_constraints_t();
        let claim = agent_claim_invade(ptr::null_mut(), constraints);
        AgentClaim {claim: claim, verbose: false}
    }

    /// An alias for the constructor
    ///
    /// # Arguments
    ///
    /// * `constraints` - The constraints of this claim
    pub fn invade(constraints: Constraints) -> AgentClaim {
        AgentClaim::new(constraints)
    }

    /// Executes an ilet on the claim
    ///
    /// # Arguments
    ///
    /// * `ilet` - The ilet to execute
    pub fn infect(&self, ilet: ilet_func) {

        let mut ilet_struct: simple_ilet = simple_ilet { padding: [0; 32] };
        simple_ilet_init(&mut ilet_struct, ilet, ptr::null_mut());

        for tile in 0..get_tile_count() {

            let pes = agent_claim_get_pecount_tile_type(self.claim, tile as u8, 0);

            if pes != 0 {
                let proxy_claim = agent_claim_get_proxyclaim_tile_type(self.claim, tile as i32, 0);

                if self.verbose {
                    unsafe {
                        printf("* Got Proxy Claim %p\n\0".as_ptr(), proxy_claim);
                    }
                }

                proxy_infect(proxy_claim, &mut ilet_struct, pes as u32);
            }
        }
    }

    /// Reinvades reusing the previous constraints
    pub fn reinvade(&self) {
        agent_claim_reinvade(self.claim);
    }

    /// Reinvades using new constraints
    ///
    /// # Arguments
    ///
    /// * `constraints` - The constraints with which to reinvade
    pub fn reinvade_with_constraints(&self, constraints: Constraints) {
        agent_claim_reinvade_constraints(self.claim, constraints.to_constraints_t());
    }

    /// Prints the Current Claim's size
    pub fn print_size(&self) {
        unsafe {
            printf("* Claim has size of %d\n\0".as_ptr(), agent_claim_get_pecount(self.claim));
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
}

/// Implements the Drop trait for the AgentClaim struct.
impl Drop for AgentClaim {

    /// Implicitly retreats when the AgentClaim struct goes out of scope
    fn drop(&mut self) {

        if self.verbose {
            unsafe { printf("* Retreating\n\0".as_ptr()); }
        }

        agent_claim_retreat(self.claim);

    }
}