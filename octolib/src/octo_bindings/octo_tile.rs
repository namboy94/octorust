/// Status information about tile and its components; conversion functions for TLM pointer

use libc::c_void;
use octo_bindings::octo_types::tile_id_t;

extern {
    #[link_name="get_cpu_id"]
    fn __get_cpu_id() -> u32;

    #[link_name="get_tile_id"]
    fn __get_tile_id() -> u32;

    #[link_name="get_tile_core_count"]
    fn __get_tile_core_count() -> u32;

    #[link_name="get_tile_count"]
    fn __get_tile_count() -> u32;

    #[link_name="get_compute_tile_count"]
    fn __get_compute_tile_count() -> u32;

    #[link_name="get_compute_tile_ids"]
    fn __get_compute_tile_ids(tids: *const u32, count: u32) -> u32;

    #[link_name="get_tlm_start"]
    fn __get_tlm_start() -> c_void;

    #[link_name="get_tlm_size"]
    fn __get_tlm_size() -> u32;

    #[link_name="get_scratchpad_size"]
    fn __get_scratchpad_size() -> u32;

    #[link_name="get_tile_id_from_ptr"]
    fn __get_tile_id_from_ptr(global_tlm_ptr: c_void) -> i32;

    #[link_name="get_global_address"]
    fn __get_global_address(local_tlm_ptr: c_void) -> c_void;

    #[link_name="get_global_address_universal"]
    fn __get_global_address_universal(ptr: c_void) -> c_void;

    #[link_name="get_global_address_for_tile"]
    fn __get_global_address_for_tile(local_tlm_ptr: c_void, tid: tile_id_t) -> c_void;

    #[link_name="get_local_address"]
    fn __get_local_address(global_tlm_ptr: c_void) -> c_void;

    #[link_name="reserve_vc"]
    fn __reserve_vc(dst_tile: tile_id_t, service_level: u16) -> i32;

    #[link_name="free_vc"]
    fn __free_vc(dst_tile: tile_id_t) -> i32;

    #[link_name="enable_coherency"]
    fn __enable_coherency(remote_tile: tile_id_t) -> i32;

    #[link_name="disable_coherency"]
    fn __disable_coherency(remote_tile: tile_id_t) -> i32;

    #[link_name="send_rpc"]
    fn __send_rpc(tid: i32);

}

/// Returns the ID of the CPU on which the current i-let is being executed.
pub fn get_cpu_id() -> u32 { unsafe { __get_cpu_id() } }

/// Get ID of current tile
pub fn get_tile_id() -> u32 { unsafe { __get_tile_id() } }

/// Get number of physical cores on current tile
pub fn get_tile_core_count() -> u32 { unsafe { __get_tile_core_count() } }

/// Gets the number of tiles in the system.
/// This number includes all compute tiles, which can be invaded, plus the I/O
/// and TCPA tiles, which cannot be invaded via regular means.
pub fn get_tile_count() -> u32 { unsafe { __get_tile_count() } }

/// Gets the number of compute tiles in the system.
///
/// This number does not include I/O or TCPA tiles.
pub fn get_compute_tile_count() -> u32 { unsafe { __get_compute_tile_count() } }

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
pub fn get_compute_tile_ids(tids: *const u32, count: u32) -> u32 {
    unsafe { __get_compute_tile_ids(tids, count) }
}

/// Returns the start address of the user-memory area located in the own TLM.
///
/// # Note
///
/// Applications should never ever write directly to this memory unless
/// they know exactly what they are doing!
pub fn get_tlm_start() -> c_void { unsafe { __get_tlm_start() } }

/// Get size of TLM in bytes
pub fn get_tlm_size() -> u32 { unsafe { __get_tlm_size() } }

/// Get size of scratchpad memory
pub fn get_scratchpad_size() -> u32 { unsafe { __get_scratchpad_size() } }

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
pub fn get_tile_id_from_ptr(global_tlm_ptr: c_void) -> i32 {
    unsafe { __get_tile_id_from_ptr(global_tlm_ptr) }
}

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
pub fn get_global_address(local_tlm_ptr: c_void) -> c_void {
    unsafe { __get_global_address(local_tlm_ptr) }
}

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
pub fn get_global_address_universal(ptr: c_void) -> c_void {
    unsafe { __get_global_address_universal(ptr) }
}

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
pub fn get_global_address_for_tile(local_tlm_ptr: c_void, tid: tile_id_t) -> c_void {
    unsafe { __get_global_address_for_tile(local_tlm_ptr, tid) }
}

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
pub fn get_local_address(global_tlm_ptr: c_void) -> c_void {
    unsafe { __get_local_address(global_tlm_ptr) }
}

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
pub fn reserve_vc(dst_tile: tile_id_t, service_level: u16) -> i32 {
    unsafe { __reserve_vc(dst_tile, service_level) }
}

/// Free a previously reserved an virtual channel on the NoC
///
/// # Arguments
///
/// * `dst_tile` - Destination tile of the virtual channel that shall be freed
///
/// # Return Value
///
/// 0 if free succeeded, -1 if free failed
pub fn free_vc(dst_tile: tile_id_t) -> i32 { unsafe { __free_vc(dst_tile) } }

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
pub fn enable_coherency(remote_tile: tile_id_t) -> i32 {
    unsafe { __enable_coherency(remote_tile) }
}

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
pub fn disable_coherency(remote_tile: tile_id_t) -> i32 {
    unsafe { __disable_coherency(remote_tile) }
}

pub fn send_rpc(tid: i32) { unsafe { __send_rpc(tid) } }
