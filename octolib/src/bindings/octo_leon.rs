/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_leon.h file
///
/// Functions only meaningful in LEON version
/// LEON specific functions for platforms with APBUART

extern {

    /// set the bus frequency of your system (in Hz)
    pub fn leon_set_bus_frequency(f: u32);

    pub fn leon_set_baud_rate(rate: u32);

    /// set FIFO Debug mode of APBUART:
    /// 0: disable debug mode; output is set through normal serial interface
    /// 1: enable debug mode; output can be viewed with grmon -u
    pub fn leon_set_uart_debug_mode(m: i32);

    /// Get NoC timestamp counter
    ///
    /// # Return Value
    ///
    /// value of the NoC's timestamp counter
    pub fn leon_get_noc_ts() -> u64;

    /// Issues a trigger condition for ChipScope debugging.
    pub fn chipscope_trigger(value: u32);

}
