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

extern { fn printf(s: *const u8, ...); }

pub fn print(text: &str) {
    unsafe {
        printf(text.as_ptr());
    }
}

pub fn print_one<T>(text: &str, value: T) {
    unsafe {
        printf(text.as_ptr(), value);
    }
}

pub fn print_two<T1, T2>(text: &str, value1: T1, value2: T2) {
    unsafe {
        printf(text.as_ptr(), value1, value2);
    }
}