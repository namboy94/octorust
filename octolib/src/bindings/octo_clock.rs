/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_clock.h file

/// Provides clock() function to measure wall time.

use octo_types::*;

extern {
    #![link_name="clock"]
    fn __clock() -> clock_t;
}

/// # Return Value
///
/// time since booting in microseconds
pub fn clock() -> clock_t {
    unsafe {
        __clock()
    }
}
