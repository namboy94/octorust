/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use libc;
use octo_structs;
use octo_signal;
use octo_ilet;
use octo_dispatch_claim;

/// A helper function that streamlines signalling reply to the agent claim using signals
///
/// # Arguments
///
/// `signal` - The signal to reply to
pub fn reply_signal(signal: *mut libc::c_void) {

    /// Internal function used to actually reply to the signal
    ///
    /// # Arguments
    ///
    /// `sig` - The signal to reply to
    extern "C" fn inner(sig: *mut libc::c_void) {
        octo_signal::simple_signal_signal_and_exit(sig as *mut octo_structs::simple_signal);
    }

    let mut answer: octo_structs::simple_ilet = octo_structs::simple_ilet {
        padding: [0; octo_structs::SIMPLE_ILET_SIZE]
    };
    octo_ilet::simple_ilet_init(&mut answer, inner, signal);
    octo_dispatch_claim::dispatch_claim_send_reply(&mut answer);
}