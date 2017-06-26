extern {
    #[link_name="get_tile_id"] fn __get_tile_id() -> u32;
}

pub fn get_tile_id() -> u32 { unsafe { __get_tile_id() } }