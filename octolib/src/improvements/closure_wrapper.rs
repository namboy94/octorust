/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

use core::mem;
use octo_types::c_void;

/// Extern C function that executes a closure by converting the closure data from
/// a void pointer back to a function and executing it.
///
/// # Arguments
///
/// `closure_data` - The closure data
/// `params` - Parameter for the closure function
pub extern "C" fn closure_handler(closure_data: *mut c_void, params: *mut c_void) {
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(closure_data) };
    closure(params);
}
