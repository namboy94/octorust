/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_guest.h file

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