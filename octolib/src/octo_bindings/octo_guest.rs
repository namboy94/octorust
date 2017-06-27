extern {
    #[link_name="shutdown"] fn __shutdown(exitcode: i32);
    #[link_name="guest_shutdown"] fn __guest_shutdown();
}

pub fn shutdown(exitcode: i32) { unsafe { __shutdown(exitcode) } }
pub fn guest_shutdown() { unsafe { __guest_shutdown() } }