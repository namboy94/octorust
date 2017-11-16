/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

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
pub extern "C" fn execute_closure(closure_data: *mut c_void, params: *mut c_void) {
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(closure_data) };
    closure(params);
}
