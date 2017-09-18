/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_gc_support.h file

/// Support functions for garbage collectors.

use octo_types::*;

extern {
    #[link_name="gc_stop_world"]
    fn __gc_stop_world() -> i32;

    #[link_name="gc_start_world"]
    fn __gc_start_world() -> i32;

    // TODO Figure out how to port
    // #[link_name="gc_iterate_all_stacks"]
    // fn __gc_iterate_all_stacks(*: void (*callback)(const void, *): const void) -> i32;

    #[link_name="gc_stop_world_on_tile"]
    fn __gc_stop_world_on_tile(tid: tile_id_t);

}

/// Stops execution on all cores on this tile except for the core on which
///        this function is called.
/// # Return Value
///
/// 0 on success, -1 if this tile is already stopped.
pub fn gc_stop_world() -> i32 {
    unsafe {
        __gc_stop_world()
    }
}

/// Resumes execution on the cores on this tile that were stopped.
/// # Return Value
///
/// 0 on success, -1 if this tile is not currently stopped.
pub fn gc_start_world() -> i32 {
    unsafe {
        __gc_start_world()
    }
}

/*  TODO Figure out how to port
/// Invokes a callback function for every active stack.
///
/// # Arguments
///
/// * `callback` - Callback function. Receives pointers to the lower and the
///                upper end of the respective stack.
/// # Return Value
///
/// 0 on success, -1 if this tile is not currently stopped.
pub fn gc_iterate_all_stacks(*: void (*callback)(const void, *): const void) -> i32 {
    unsafe {
        __gc_iterate_all_stacks(*, *))
    }
}
*/

/// This is just for debugging purposes!
pub fn gc_stop_world_on_tile(tid: tile_id_t) {
    unsafe {
        __gc_stop_world_on_tile(tid)
    }
}
