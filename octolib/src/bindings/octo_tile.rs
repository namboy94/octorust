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
/// This fle is based on the IRTSS octo_tile.h file
///
/// Status information about tile and its components; conversion functions for TLM pointer

use octo_types::*;

extern {

    /// Returns the ID of the CPU on which the current i-let is being executed.
    pub fn get_cpu_id() -> u32;

    /// Get ID of current tile
    pub fn get_tile_id() -> u32;

    /// Get number of physical cores on current tile
    pub fn get_tile_core_count() -> u32;

    /// Gets the number of tiles in the system.
    /// This number includes all compute tiles, which can be invaded, plus the I/O
    /// and TCPA tiles, which cannot be invaded via regular means.
    pub fn get_tile_count() -> u32;

    /// Gets the number of compute tiles in the system.
    ///
    /// This number does not include I/O or TCPA tiles.
    pub fn get_compute_tile_count() -> u32;

    /// Returns the IDs of all compute tiles.
    ///
    /// # Arguments
    ///
    /// * `tids` - Pointer to a caller-allocated array receiving the compute-tile IDs.
    /// * `count` - Size of the tile-ID array as provided by the caller.
    ///
    /// # Return Value
    ///
    /// Number of compute-tile IDs that were written into the array.
    ///
    /// # Note
    ///
    /// A caller should first call get_compute_tile_count() to determine the required
    /// size for the array.
    /// If count is less than the actual number of compute tiles present in the
    /// system, only the IDs of the first count compute tiles will be stored into the
    /// array.
    pub fn get_compute_tile_ids(tids: *const u32, count: u32) -> u32;

    /// Returns the start address of the user-memory area located in the own TLM.
    ///
    /// # Note
    ///
    /// Applications should never ever write directly to this memory unless
    /// they know exactly what they are doing!
    pub fn get_tlm_start() -> c_void;

    /// Get size of TLM in bytes
    pub fn get_tlm_size() -> u32;

    /// Get size of scratchpad memory
    pub fn get_scratchpad_size() -> u32;

    /// Given a global pointer to a TLM location this function returns the
    /// tile it belongs to.
    ///
    /// # Arguments
    ///
    /// * `global_tlm_ptr` - Global pointer to a tlm location.
    ///
    /// # Return Value
    ///
    /// Returns the tile id the memory belongs to or -1 if memory address is invalid.
    pub fn get_tile_id_from_ptr(global_tlm_ptr: c_void) -> i32;

    /// Get the global address of a pointer in TLM. Calculation is done
    /// relative to the current tile ID. This operation is a nop on the guest layer.
    ///
    /// According to the proposed InvasIC memory map, TLM is mapped twice. To get the
    /// globally visible address of a piece of memory in TLM, use this function.
    ///
    /// # Arguments
    ///
    /// * `local_tlm_ptr` - must be pointer to a memory location in the local TLM
    ///                     that is NOT globally visible
    ///
    /// # Return Value
    ///
    /// Returns a pointer to the same physical memory location, but now
    /// inside the globally visible part of the address space
    pub fn get_global_address(local_tlm_ptr: c_void) -> c_void;

    /// Get the global address of a pointer in TLM. Calculation is done
    /// relative to the current tile ID. This operation is a nop on the guest layer.
    ///
    /// According to the proposed InvasIC memory map, TLM is mapped twice. To get the
    /// globally visible address of a piece of memory in TLM, use this function.
    ///
    /// # Arguments
    ///
    /// * `ptr` - can be a pointer to any memory location
    ///
    /// # Return Value
    ///
    /// If the parameter **ptr** was a pointer to the local TLM, a pointer to
    /// the same physical memory location, but now inside the globally visible part
    /// of the address space is returned. Otherwise **ptr** is returned, so the
    /// operation is a nop.
    pub fn get_global_address_universal(ptr: c_void) -> c_void;

    /// Get the global address of a pointer in TLM. Calculation is done
    /// relative to the provided tile id. This operation is a nop on the guest layer.
    ///
    /// **BEWARE** This operation in conjunction with get_tile_id_from_ptr has to be
    /// used with care. It is a nop on the guest layer (as all TLM addresses are
    /// global by default in the guest layer). So calling get_tile_id_from_ptr on
    /// the original **local_tlm_ptr** and on the result yields the same tile in the
    /// guest layer. On the LEON, however, calling get_tile_id_from_ptr on
    /// **local_tlm_ptr** will yield an error code (-1), as it only works on global
    /// addresses. Calling get_tile_id_from_ptr on the result of this function will
    /// yield \b tid.
    ///
    /// A valid use case of this function would be to convert local pointers
    /// allocated on other tiles to their global representation with respect to the
    /// tile they originally were allocated on.
    ///
    /// # Arguments
    ///
    /// * `local_tlm_ptr` - must be pointer to a memory location in the local TLM
    ///                     range that is NOT globally visible
    /// * `tid` - tile id of the tile where **local_tlm_ptr** belongs to
    ///
    /// # Return Value
    ///
    /// Returns a global pointer to the TLM location referencing it on tile **tid**
    pub fn get_global_address_for_tile(local_tlm_ptr: c_void, tid: tile_id_t) -> c_void;

    /// Get the local address of a pointer in TLM. This operation is a nop on
    /// the guest layer.
    ///
    /// According to the proposed InvasIC memory map, TLM is mapped twice. To get the
    /// locally visible address from a pointer that points to the globally visible
    /// address of the same piece of memory, use this function.
    ///
    /// This will work with pointers translated through get_global_address and
    /// get_global_address_for_tile.
    ///
    /// # Arguments
    ///
    /// * `global_tlm_ptr` - pointer to a TLM location of the current Tile in the
    ///                      globally visible part of the address space
    ///
    /// # Return Value
    ///
    /// returns a pointer to the same physical memory location, but now
    /// inside the locally visible part of the address space
    pub fn get_local_address(global_tlm_ptr: c_void) -> c_void;

    /// Reserve an virtual channel on the NoC
    ///
    /// # Arguments
    ///
    /// * `dst_tile` - Destination tile of the virtual channel
    /// * `service_level` - non-zero 16-bit value: for now use low values like 1 or 2
    ///
    /// # Return Value
    ///
    /// 0 if reservation succeeded, -1 if reservation failed
    pub fn reserve_vc(dst_tile: tile_id_t, service_level: u16) -> i32;

    /// Free a previously reserved an virtual channel on the NoC
    ///
    /// # Arguments
    ///
    /// * `dst_tile` - Destination tile of the virtual channel that shall be freed
    ///
    /// # Return Value
    ///
    /// 0 if free succeeded, -1 if free failed
    pub fn free_vc(dst_tile: tile_id_t) -> i32;

    /// Enable cache coherency for memory on a remote tile
    ///
    /// # Note
    ///
    /// This is a dummy function. Multi-tile cache coherency is not yet implemented!
    /// The interface to control cache coherency might change in the future.
    ///
    /// # Arguments
    ///
    /// * `remote_tile` - Remote tile
    ///
    /// # Return Value
    ///
    /// 0 if succeeded, -1 if failed
    pub fn enable_coherency(remote_tile: tile_id_t) -> i32;

    /// Disable cache coherency for memory on a remote tile
    ///
    /// # Note
    ///
    /// This is a dummy function. Multi-tile cache coherency is not yet implemented!
    /// The interface to control cache coherency might change in the future.
    ///
    /// # Arguments
    ///
    /// * `remote_tile` - Remote tile
    ///
    /// # Return Value
    ///
    /// 0 if succeeded, -1 if failed
    pub fn disable_coherency(remote_tile: tile_id_t) -> i32;
    
    pub fn send_rpc(tid: i32);

}
