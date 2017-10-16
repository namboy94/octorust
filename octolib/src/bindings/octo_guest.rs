/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_guest.h file

extern {

    /// Deprecated. Use shutdown(int exitcode) instead.
    pub fn shutdown(exitcode: i32);

    /// Shut down and print the exit code to stdout
    pub fn guest_shutdown();
}
