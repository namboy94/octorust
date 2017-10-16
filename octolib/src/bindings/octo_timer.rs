/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_timer.h file

use octo_types::*;

extern {
    
    pub fn timer_start() -> uint64_t;

    pub fn timer_stop() -> uint64_t;

    pub fn ticks_to_nanoseconds(ticks: uint64_t) -> uint64_t;

}
