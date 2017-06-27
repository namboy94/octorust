extern {
    #[link_name="shutdown"]
    fn __shutdown(exitcode: i32);

    #[link_name="guest_shutdown"]
    fn __guest_shutdown();
}

/// Deprecated. Use shutdown(int exitcode) instead.
pub fn shutdown(exitcode: i32) { unsafe { __shutdown(exitcode) } }


/// Shut down and print the exit code to stdout
pub fn guest_shutdown() { unsafe { __guest_shutdown() } }