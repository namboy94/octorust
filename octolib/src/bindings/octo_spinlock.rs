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
/// This fle is based on the IRTSS octo_spinlock.h file
///
/// Operations on spinlocks

use octo_structs::*;

extern {
    
    pub fn simple_spinlock_init(instance: *mut simple_spinlock);

    pub fn simple_spinlock_lock(instance: *mut simple_spinlock);

    pub fn simple_spinlock_unlock(instance: *mut simple_spinlock);

    pub fn simple_spinlock_trylock(instance: *mut simple_spinlock) -> i32;

    pub fn simple_spinlock_isLocked(instance: *mut simple_spinlock) -> i32;

    pub fn recursive_spinlock_init(instance: *mut recursive_spinlock);

    pub fn recursive_spinlock_lock(instance: *mut recursive_spinlock);

    pub fn recursive_spinlock_trylock(instance: *mut recursive_spinlock) -> i32;

    pub fn recursive_spinlock_isLocked(instance: *mut recursive_spinlock) -> i32;

    pub fn recursive_spinlock_unlock(instance: *mut recursive_spinlock);

}
