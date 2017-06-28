//use libc::{c_void};
use octolib::octo_bindings::octo_tile::*;

pub fn test_octo_tile() {
    let cpu_id: u32 = get_cpu_id();
    let tile_id: u32 = get_tile_id();
    let tile_core_count: u32 = get_tile_core_count();
    let tile_count: u32 = get_tile_count();
    let compute_tile_count: u32 = get_compute_tile_count();
    //let tlm_start: c_void = get_tlm_start();
    let tlm_size: u32 = get_tlm_size();
    let scratchpad_size: u32 = get_scratchpad_size();
}


/*

    #[link_name="get_compute_tile_ids"]
    fn __get_compute_tile_ids(tids: *const u32, count: u32) -> u32;


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
*/