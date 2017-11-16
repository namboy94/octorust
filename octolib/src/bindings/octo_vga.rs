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
/// This fle is based on the IRTSS octo_vga.h file
///
/// VGA output on guest layer

extern {

    /// Set the color of a pixel in the virtual VGA framebuffer
    /// Sets the color of a pixel at coordinate (\b x,\b y) to the RGB values
    /// provided
    pub fn vga_set_pixel(x: i32, y: i32, r: i32, g: i32, b: i32);

    /// Update the screen to show previous changes
    pub fn vga_update();

    /// Do a bitblit into the buffer of the virtual VGA device
    pub fn vga_write_buffer(buffer: *mut u32);

    pub fn vga_get_framebuffer() -> *mut u32;

}
