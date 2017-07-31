/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Import printf for printing methods
extern { fn printf(s: *const u8, ...); }

// Imports
use libc;
use core::ptr;
use core::mem;
use helper::printer::print_text;
use improvements::constraints::Constraints;
use octo_agent::{agent_claim_invade, agent_claim_retreat, agent_claim_reinvade,
                 agent_claim_reinvade_constraints, agent_claim_get_pecount_tile_type,
                 agent_claim_get_proxyclaim_tile_type, agent_claim_get_pecount};
use octo_types::{agentclaim_t, ilet_func, c_void};
use octo_structs::{simple_ilet, simple_signal};
use octo_ilet::{simple_ilet_init};
use octo_proxy_claim::{proxy_infect};
use octo_tile::{get_tile_count};
use octo_signal::{simple_signal_wait, simple_signal_init};

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

    /// Executes an ilet on the claim without using signals
    ///
    /// # Arguments
    ///
    /// * `ilet` - The ilet function to execute
    pub fn infect(&mut self, ilet: ilet_func) {
        unsafe { self.initialize_proxy_infect(ilet, None); };
    }


    /// Executes an ilet on the claim using signals
    ///
    /// # Arguments
    ///
    /// * `ilet` - The ilet function to execute
    ///
    /// # Return Value
    ///
    /// The signal used while initialising the ilets
    pub fn infect_signal(&mut self, ilet: ilet_func) -> simple_signal {

        let mut sync = simple_signal { padding: [0; 64] };
	    simple_signal_init(&mut sync, agent_claim_get_pecount(self.claim) as usize);

        unsafe { sync = self.initialize_proxy_infect(ilet, Some(sync)).unwrap() }
        return sync;
    }

    /// Instanciates simple_ilet structs for every PE on the claim and infects with them
    /// using a proxy claim
    ///
    /// # Arguments
    ///
    /// * `ilet` - The ILet function
    /// * `sync` - The signal with which to instanciate the simple_ilets
    unsafe fn initialize_proxy_infect(&mut self, ilet: ilet_func, mut sync: Option<simple_signal>) -> Option<simple_signal> {

        for tile in 0..get_tile_count() {
            let pes = agent_claim_get_pecount_tile_type(self.claim, tile as u8, 0);
            if pes != 0 {  // Type = 0 ^= RISC

                let proxy_claim = agent_claim_get_proxyclaim_tile_type(self.claim, tile as i32, 0);

                if self.verbose { printf("* Got Proxy Claim %p\n\0".as_ptr(), proxy_claim); }

                let array_size = mem::size_of::<simple_ilet>() * pes as usize;
                let ilets: *mut simple_ilet = libc::malloc(array_size) as *mut simple_ilet;

                for i in 0..pes as isize {
                    sync = match sync {
                        Some(mut s) => { simple_ilet_init(ilets.offset(i), ilet, &mut s as *mut _ as *mut c_void); Some(s)},
                        None => { simple_ilet_init(ilets.offset(i), ilet, ptr::null_mut()); None}
                    }

                }

                proxy_infect(proxy_claim, ilets, pes as u32);

                if self.verbose { printf("Infecting %d Ilets on Tile %d\n\0".as_ptr(), pes, tile); }
            }
        }
        return sync;
    }


    /// Executes an ilet on the claim using signals and waits for their completion
    ///
    /// # Arguments
    ///
    /// * `ilet` - The ilet function to execute
    pub fn infect_signal_wait(&mut self, ilet: ilet_func) {

        let mut sync = self.infect_signal(ilet);

        if self.verbose {
            unsafe { printf("Waiting on Signal %p...\n\0".as_ptr(), &mut sync); }
        }

        simple_signal_wait(&mut sync);

        if self.verbose {
            print_text("All Signals received!\n\0");
        }

    }

    /// Reinvades reusing the previous constraints
    pub fn reinvade(&mut self) {
        let status = agent_claim_reinvade(self.claim);
        if status == -1 {
            unsafe { printf("* Reinvade Failed\n\0".as_ptr()); }
        } else {
            unsafe { printf("* Reinvade Successful\n\0".as_ptr()); }
        }
    }

    /// Reinvades using new constraints
    ///
    /// # Arguments
    ///
    /// * `constraints` - The constraints with which to reinvade
    pub fn reinvade_with_constraints(&mut self, constraints: Constraints) {
        agent_claim_reinvade_constraints(self.claim, constraints.to_constraints_t());
    }

    /// Prints the Current Claim's size
    pub fn print_size(&self) {
        unsafe {
            printf("* Claim has size of %d\n\0".as_ptr(), agent_claim_get_pecount(self.claim));
        }
    }

    /// Enables or Disables printing of messages when the claim infects, retreats, etc.
    ///
    /// # Arguments
    ///
    /// * `verbose` - Sets the verbosity of the claim
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