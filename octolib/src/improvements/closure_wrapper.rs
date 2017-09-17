/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

use core::mem;
use octo_types::{c_void, proxy_claim_t};
use octo_structs::simple_ilet;
use octo_ilet::dual_ilet_init;

extern {
    fn c_closure_infect(pes: usize, proxy_claim: proxy_claim_t, ilet: extern fn(*mut c_void, *mut c_void), closure_data: *mut c_void, params: *mut c_void);
}

/// Initializes an ilet using a Rust closure.
/// Only |x: *mut c_void| closures are allowed sadly, limited by dual_ilet_init
pub fn closure_infect<F>(
    pes: usize,
    proxy_claim: proxy_claim_t,
    mut ilet: F,
    params: *mut c_void
) where F: FnMut(*mut c_void) {

    // Convert closure
    let mut cb: &mut FnMut(*mut c_void) = &mut ilet;
	let ctx = &mut cb as *mut &mut FnMut(*mut c_void) as *mut c_void;
    let clos: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(ctx) };

    unsafe {
        c_closure_infect(pes, proxy_claim, closure_handler, clos as *mut _ as *mut c_void, params);
    }
}

/// Handles the execution of the closure
extern "C" fn closure_handler(closure_data: *mut c_void, params: *mut c_void) {
    // Execute the C closure after re-converting the closure
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(closure_data) };
    closure(params);
}