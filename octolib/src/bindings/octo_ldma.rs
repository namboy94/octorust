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
/// This fle is based on the IRTSS octo_ldma.h file
///
/// Local DMA engine.

use octo_types::*;
use octo_structs::*;

extern {

    /// Copy memory using the local DMA engine.
    /// The sync_dma_transfer() function copies len bytes from memory area src to memory area dest.
    ///
    /// # Arguments
    ///
    /// * `src` - A pointer to the source buffer
    /// * `dest` - A pointer to the destionation buffer
    /// * `len` - the number of bytes to copy.
    pub fn sync_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32);

    /// Asynchronously copy memory using the local DMA engine.
    /// The async_ldma_copy() functions copies len bytes from memory area
    /// src to the memorey area dest. You can await the completion of the
    /// operation using the syscall_future API.
    ///
    /// # Arguments
    ///
    /// * `src` - A pointer to the source buffer.
    /// * `dest` - A pointer to the destination buffer.
    /// * `len` - the number of bytes to copy.
    pub fn async_ldma_copy(dest: *mut c_void, src: *const c_void, len: u32, future: *mut syscall_future);

    /// Asynchronously copy memory using the local DMA engine.
    /// The async_dma_transfer() function copies len bytes from memory area
    /// src to memory area dest and schedules ilet when the copy is
    /// finished.
    ///
    /// # Arguments
    ///
    /// * `src` - A pointer to the source buffer
    /// * `dest` - A pointer to the destionation buffer
    /// * `len` - the number of bytes to copy.
    /// * `ilet` - a pointer to an ilet
    pub fn async_ldma_copy_ilet(dest: *mut c_void, src: *const c_void, len: u32, ilet: *mut simple_ilet);

    /// Asynchronously copy memory using the local DMA engine.
    /// The async_dma_transfer() function copies len bytes from memory area
    /// src to memory area dest and schedules ilet when the copy is
    /// finished.
    ///
    /// # Arguments
    ///
    /// * `src` - A pointer to the source buffer
    /// * `dest` - A pointer to the destionation buffer
    /// * `len` - the number of bytes to copy.
    /// * `ilet` - a pointer to an ilet
    ///
    /// # Deprecated
    ///
    /// use async_ldma_copy() instead.
    pub fn async_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32, ilet: *mut simple_ilet);

    /// Set memory using the local DMA engine.
    /// The sync_dma_memset() function set len bytes of the memory area dst to the value val.
    ///
    /// # Arguments
    ///
    /// * `dst` - A pointer to the destionation buffer
    /// * `val` - The value to set
    /// * `len` - the number of bytes to set
    pub fn sync_dma_memset(dst: *mut c_void, val: u32, len: u32);

}
