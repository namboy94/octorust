/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

use core::mem;
use octo_types::c_void;

pub extern "C" fn closure_handler(closure_data: *mut c_void, params: *mut c_void) {
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(closure_data) };
    closure(params);
}

pub fn convert_closure<F>(mut closure: F) -> *mut c_void where F: FnMut(*mut c_void) {
    let mut callback: &mut FnMut(*mut c_void) = &mut closure;
    let ctx = &mut callback as *mut &mut FnMut(*mut c_void) as *mut c_void;
    let closure_data: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(ctx) };
    return closure_data as *mut _ as *mut c_void
}