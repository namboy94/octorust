/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

use core::mem;
use octo_types::{c_void};

extern "C" {
    fn handle_closure_in_c(
        f: Option<extern "C" fn(closure_data: *mut c_void, params: *mut c_void)>,
        closure_data: *mut c_void, params:
        *mut c_void
    );
}

extern "C" fn closure_handler(closure_data: *mut c_void, params: *mut c_void) {
    // Execute the C closure after re-converting the closure
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(closure_data) };
    closure(params);
}

pub fn run_closure_in_c<F>(mut callback: F, params: *mut c_void)
    where F: FnMut(*mut c_void) {

    // Convert closure
    let mut cb: &mut FnMut(*mut c_void) = &mut callback;
	let ctx = &mut cb as *mut &mut FnMut(*mut c_void) as *mut c_void;
    // let cb2: *mut *mut FnMut(*mut c_void) = unsafe { mem::transmute(ctx) };
    let clos: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(ctx) };

    unsafe {
        handle_closure_in_c(
            Some(closure_handler),
            clos as *mut _ as *mut c_void,
            params
        );
    }
}