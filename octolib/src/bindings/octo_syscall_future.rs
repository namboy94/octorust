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
/// This fle is based on the IRTSS octo_syscall_future.h file

use octo_types::*;
use octo_structs::*;

extern {

    /// Initialize a future.
    ///
    /// # Arguments
    ///
    /// * `future` - A pointer to the future to initialize.
    pub fn syscall_future_init(future: *mut syscall_future);

    /// Initialize a future.
    ///
    /// # Arguments
    ///
    /// * `future` - A pointer to the future to initialize.
    /// * `ret_val` - A pointer to the memory area where the ret_val should be placed.
    pub fn syscall_future_init_ret_val(future: *mut syscall_future, ret_val: *mut c_void);

    /// Add a "on completion" iLet to the given future.
    /// The given ilet will be execute once the future has completed. It's
    /// save to add this ilet after the future has already been completed.
    ///
    /// # Arguments
    ///
    /// * `future` - A pointer to the future.
    /// * `ilet` - The "on completion" ilet.
    pub fn syscall_future_on_completion(future: *mut syscall_future, ilet: *mut simple_ilet);

    /// Await the completion of the future.
    /// This function block until the future is completed.
    ///
    /// # Arguments
    ///
    /// * `future` - A pointer to the future.
    pub fn syscall_future_wait(future: *mut syscall_future);

}
