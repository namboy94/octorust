/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_debug.h file

extern {
    #![link_name="debug_print_blocked_ilets"]
    fn __debug_print_blocked_ilets();
}

pub fn debug_print_blocked_ilets() {
    unsafe {
        __debug_print_blocked_ilets()
    }
}