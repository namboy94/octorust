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
/// This fle is based on the IRTSS octo_cilk_support.h file
///
/// Support functions for the Cilk Plus runtime system.

extern {

    /// Creates and enqueues a continuation.
    /// This function must be called before cilk_yield(). Calling
    /// simple_signal_wait() or any other blocking system call between
    /// cilk_create_continuation() and cilk_yield() leads to undefined behaviour.
    pub fn cilk_create_continuation();

    /// Relinquishes the CPU, allowing another i-let to run.
    /// If the current i-let is ever supposed to be resumed,
    /// cilk_create_continuation() must be called before.
    pub fn cilk_yield();

    /*
    /// Spawns a function in a new i-let.
    /// The current control-flow state is saved and a continuation is inserted into
    /// the scheduler queue.
    */
    // TODO void cilk_spawn_function(void (*func)(void *, void *), void *arg1, void *arg2);
}
