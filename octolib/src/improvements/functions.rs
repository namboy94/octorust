/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use octo_types;
use octo_structs;
use bindings::octo_signal;
use bindings::octo_ilet;
use bindings::octo_dispatch_claim;

/// A helper function that streamlines signalling reply to the agent claim using signals
///
/// # Arguments
///
/// `signal` - The signal to reply to
pub fn reply_signal(signal: *mut octo_types::c_void) {

    /// Internal function used to actually reply to the signal
    ///
    /// # Arguments
    ///
    /// `sig` - The signal to reply to
    extern "C" fn inner(sig: *mut octo_types::c_void) {
        unsafe {
            octo_signal::simple_signal_signal_and_exit(sig as *mut octo_structs::simple_signal);
        }
    }

    let mut answer: octo_structs::simple_ilet = octo_structs::simple_ilet {
        padding: [0; octo_structs::SIMPLE_ILET_SIZE]
    };
    unsafe {
        octo_ilet::simple_ilet_init(&mut answer, inner, signal);
        octo_dispatch_claim::dispatch_claim_send_reply(&mut answer);
    }
}