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
/// This fle is based on the IRTSS octo_fls.h file
///
/// Fibre-local-storage functions.
///
/// "Fibre" is another name for an i-let in execution. Fibre-local storage
/// enables the programmer to store small amounts of data (usually pointers) in a
/// memory area whose lifespan is identical to the lifespan of the fibre. The
/// values are preserved across blocking operations.
///
/// The programming interface is very similar to the Windows API.
///
/// Fibre-local storage supersedes the old "i-let-local data" interface, which
/// has imprecise semantics and is awkward to use by more than one program
/// component.

use octo_types::*;

extern {

    /*
    /// Allocates a fibre-local-storage index.
    /// Any fibre on this tile can subsequently use the returned index to store and
    /// retrieve values that are local to the fibre. If the allocation fails, -1 is
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `destructor` - An optional pointer to a callback function. If the
    ///                  associated slot is in use (i.e. non-NULL), this function will be
    ///                  called in the following situations:
    ///                  - A fibre finishes execution.
    ///                  - A fibre-local-storage index is freed.
    */
    // TODO intptr_t fls_allocate(void (*destructor)(void *));

    /// Releases a fibre-local-storage index, making it available for reuse on
    /// this tile.
    ///
    /// # Return Value
    ///
    /// 0 on success, -1 on error.
    pub fn fls_free(index: usize) -> i32;

    /// Stores a pointer value in the calling i-let's fibre-local-storage slot
    /// for the specified index.
    ///
    /// # Return Value
    ///
    /// 0 on success, -1 on error.
    pub fn fls_set(index: usize, value: *mut c_void) -> i32;

    /// Retrieves a pointer value previously stored in the calling i-let's
    /// fibre-local-storage slot for the specified index. If the slot is
    /// uninitialised, NULL is returned.
    pub fn fls_get(index: usize) -> *mut c_void;

}
