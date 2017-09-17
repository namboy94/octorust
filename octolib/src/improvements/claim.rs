/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use libc;
use core::ptr;
use core::mem;
use octo_agent;
use octo_types;
use octo_structs;
use octo_ilet;
use octo_proxy_claim;
use octo_tile;
use octo_signal;
use improvements::constraints::Constraints;
use improvements::closure_wrapper::closure_handler;
use improvements::functions::reply_signal;
extern {
    fn printf(s: *const u8, ...);
    fn simple_ilet_init(ilet: *mut octo_structs::simple_ilet,
                        code: octo_types::rust_ilet_func,
                        param: *mut octo_types::c_void);  // To allow rust fns to be used
    fn closure_infect(claim: octo_types::agentclaim_t, ilet: extern fn(*mut octo_types::c_void, *mut octo_types::c_void), closure_data: *mut octo_types::c_void, param: *mut octo_types::c_void);
}

/// The AgentClaim struct wraps around an agentclaim_t object to offer
/// a simplified interface to methods associated with an agent claim, implementing
/// an infect, reinvade and implicit retreat methods. The constructor effectively does an invade
/// operation
///
/// # Members
///
/// * `claim` - The agentclaim_t struct that this struct wraps around
/// * `constraints` - The current constraints for the claim.
///                   Gets deleted once the agent goes out of scope
/// * `verbose` - Can be set to enable printing of the agent's state
pub struct AgentClaim {
    claim: octo_types::agentclaim_t,
    constraints: octo_types::constraints_t,
    verbose: bool
}

/// Implementation of the AgentClaim struct
impl AgentClaim {
    /// Constructor for the AgentClaim struct. Automatically invades
    /// using the specified constraints
    ///
    /// # Arguments
    ///
    /// * `constraints` - The constraints of this claim. Is consumed by this method.
    pub fn new(constraints: Constraints) -> AgentClaim {
        // TODO Maybe find a way to NOT consume the constraints object
        let constr = constraints.to_constraints_t();
        let claim = octo_agent::agent_claim_invade(ptr::null_mut(), constr);
        AgentClaim { claim: claim, constraints: constr, verbose: false }
    }

    /// Enables or Disables printing of messages when the claim infects, retreats, etc.
    ///
    /// # Arguments
    ///
    /// * `verbose` - Sets the verbosity of the claim
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Reinvades using the existing constraints
    pub fn reinvade(&mut self, constraints: Option<Constraints>) {

        let status;
        match constraints {
            Some(c) => {
                octo_agent::agent_constr_delete(self.constraints);
                self.constraints = c.to_constraints_t();
                status = octo_agent::agent_claim_reinvade_constraints(self.claim, self.constraints);
            }
            None => {
                status = octo_agent::agent_claim_reinvade(self.claim);
            }
        };

        if self.verbose {
            if status == -1 {
                unsafe { printf("* Reinvade Failed\n\0".as_ptr()); }
            } else {
                unsafe { printf("* Reinvade Successful\n\0".as_ptr()); }
            }
        }
    }

    /// Executes an Ilet
    ///
    /// # Arguments
    ///
    /// `ilet` - The ilet function/closure to execute
    /// `async` - Specifies if the infect will be done asynchronously or not
    fn infecter<F>(&self, mut ilet: F, async: bool, data: Option<&[*mut octo_types::c_void]>) -> octo_structs::simple_signal
        where F: FnMut(*mut octo_types::c_void) {

        let mut sync = octo_structs::simple_signal { padding: [0; 64] };
        let pe_count = octo_agent::agent_claim_get_pecount(self.claim) as usize;
        octo_signal::simple_signal_init(&mut sync, pe_count);
        let signal_ptr = &mut sync as *mut _ as *mut libc::c_void;

        let mut closure_wrap = |param: *mut octo_types::c_void| {
            ilet(param);
            reply_signal(signal_ptr);
        };

        // Convert closure to raw data
        let mut callback: &mut FnMut(*mut octo_types::c_void) = &mut closure_wrap;
        let ctx = &mut callback as *mut &mut FnMut(*mut octo_types::c_void) as *mut octo_types::c_void;
        let closure_data: &mut &mut FnMut(*mut octo_types::c_void) = unsafe { mem::transmute(ctx) };
        let mut closure_ptr = closure_data as *mut _ as *mut octo_types::c_void;

        for tile in 0..octo_tile::get_tile_count() {
            let pes = octo_agent::agent_claim_get_pecount_tile_type(self.claim, tile as u8, 0);

            if pes != 0 { // Type = 0 ^= RISC

                let proxy_claim = octo_agent::agent_claim_get_proxyclaim_tile_type(
                    self.claim, tile as i32, 0);

                unsafe {
                    if self.verbose {
                        printf("* Got Proxy Claim %p\n\0".as_ptr(), proxy_claim);
                        printf("* Starting Infecting\n\0".as_ptr());
                    }

                    // malloc/free because we can't initialize arrays like in C
                    let arraysize = pes as usize * mem::size_of::<octo_structs::simple_ilet>();
                    let ilets: *mut octo_structs::simple_ilet =
                        libc::malloc(arraysize) as *mut octo_structs::simple_ilet;

                    for i in 0..pes as isize {

                        let mut param = match &data {
                            &Some(ref d) => d[i as usize],
                            &None => ptr::null_mut()
                        };

                        octo_ilet::dual_ilet_init(ilets.offset(i), closure_handler, closure_ptr, param)
                    }
                    octo_proxy_claim::proxy_infect(proxy_claim, ilets.offset(0), pes as u32);
                    libc::free(ilets as *mut _ as *mut libc::c_void);
                }
            }
        }

        if !async {
            if self.verbose {
                unsafe { printf("Waiting on Signal %p...\n\0".as_ptr(), &mut sync); }
            }
            octo_signal::simple_signal_wait(&mut sync);
            if self.verbose {
                unsafe { printf("All Signals received!\n\0".as_ptr()); }
            }
        }
        return sync;
    }

    pub fn infect<F>(&self, mut ilet: F, data: Option<&[*mut octo_types::c_void]>) where F: FnMut(*mut octo_types::c_void) {
        self.infecter(ilet, false, data);
    }

    pub fn infect_async<F>(&self, mut ilet: F, data: Option<&[*mut octo_types::c_void]>) -> octo_structs::simple_signal
        where F: FnMut(*mut octo_types::c_void) {
        self.infecter(ilet, true, data)
    }
}

/// Implements the Drop trait for the AgentClaim struct.
impl Drop for AgentClaim {

    /// Implicitly retreats when the AgentClaim struct goes out of scope
    fn drop(&mut self) {

        if self.verbose {
            unsafe { printf("* Retreating and deleting constraints\n\0".as_ptr()); }
        }
        octo_agent::agent_constr_delete(self.constraints);
        octo_agent::agent_claim_retreat(self.claim);
    }
}
