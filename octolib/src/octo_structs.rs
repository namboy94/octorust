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
/// This fle is based on various IRTSS header files

use octo_types::*;

#[cfg(target_arch = "x86")]
pub const SIMPLE_ILET_SIZE: usize = 16usize;

#[cfg(target_arch = "x86_64")]
pub const SIMPLE_ILET_SIZE: usize = 32usize;

#[cfg(target_arch = "sparc")]
pub const SIMPLE_ILET_SIZE: usize = 16usize;

pub const INVADE_FUTURE_SIZE: usize = 64usize;
pub const REINVADE_FUTURE_SIZE: usize = 64usize;
pub const RETREAT_FUTURE_SIZE: usize = 64usize;
pub const SYSCALL_FUTURE_SIZE: usize = 64usize;
pub const DISPATCH_CLAIM_SIZE: usize = 64usize;
pub const SIMPLE_SIGNAL_SIZE: usize = 64usize;
pub const BINARY_SIGNAL_SIZE: usize = 64usize;
pub const INFECT_SIGNAL_SIZE: usize = 64usize;
pub const SIMPLE_SPINLOCK_SIZE: usize = 64usize;
pub const RECURSIVE_SPINLOCK_SIZE: usize = 64usize;
pub const TCPA_INVADE_FUTURE_SIZE: usize = 64usize;
pub const TCPA_INFECT_FUTURE_SIZE: usize = 64usize;
pub const TCPA_GET_OUTPUT_FUTURE_SIZE: usize = 64usize;

/// struct dispatch_claim
/// Opaque type for dispatch_claim
///
/// This type is used to allocate dispatch_claim "objects" with the right size.
#[repr(C)]
pub struct dispatch_claim {
    pub padding: [c_char; DISPATCH_CLAIM_SIZE],
}
pub type dispatch_claim_t = dispatch_claim;

/// struct simple_ilet
/// Opaque type for simple_ilet
///
/// This type is used to allocate simple_ilet "objects" with the right size.
/// To initialise a variable of this type see function simple_ilet_init().
#[repr(C)]
pub struct simple_ilet {
    pub padding: [c_char; SIMPLE_ILET_SIZE],
}

/// struct invade_future
/// Opaque type for invade_future
///
/// This type is used to allocate invade_future "objects" with the right size.
#[repr(C)]
pub struct invade_future {
    pub padding: [c_char; INVADE_FUTURE_SIZE],
}
pub type invade_future_t = invade_future;

/// struct reinvade_future
/// Opaque type for reinvade_future
///
/// This type is used to allocate reinvade_future "objects" with the right size.
#[repr(C)]
pub struct reinvade_future {
    pub padding: [c_char; REINVADE_FUTURE_SIZE],
}
pub type reinvade_future_t = reinvade_future;

#[repr(C)]
pub struct retreat_future {
    pub padding: [c_char; RETREAT_FUTURE_SIZE],
}
pub type retreat_future_t = retreat_future;

/// struct simple_signal
/// Opaque type for simple_signal.
///
/// This type is used to allocate simple_signal "objects" with the right size.
/// To initialise a variable of this type, see function simple_signal_init().
#[repr(C)]
pub struct simple_signal {
    pub padding: [c_char; SIMPLE_SIGNAL_SIZE],
}

/// struct infect_signal
/// Opaque type for infect_signal.
///
/// This type is used to allocate infect_signal "objects" with the right size.
/// To initialise a variable of this type, see function infect_signal_init().
#[repr(C)]
pub struct infect_signal {
    pub padding: [c_char; INFECT_SIGNAL_SIZE],
}

/// struct binary_signal
/// Opaque type for binary_signal.
///
/// This type is used to allocate binary_signal "objects" with the right size.
/// To initialise a variable of this type, see function binary_signal_init().
#[repr(C)]
pub struct binary_signal {
    pub padding: [c_char; BINARY_SIGNAL_SIZE],
}

#[repr(C)]
pub struct simple_spinlock {
    pub padding: [c_char; SIMPLE_SPINLOCK_SIZE],
}

#[repr(C)]
pub struct recursive_spinlock {
    pub padding: [c_char; RECURSIVE_SPINLOCK_SIZE],
}

#[repr(C)]
pub struct syscall_future {
    pub padding: [c_char; SYSCALL_FUTURE_SIZE],
}

#[repr(C)]
pub struct tcpa_invade_future {
    pub padding: [c_char; TCPA_INVADE_FUTURE_SIZE],
}
pub type tcpa_invade_future_t = tcpa_invade_future;

#[repr(C)]
pub struct tcpa_infect_future {
    pub padding: [c_char; TCPA_INFECT_FUTURE_SIZE],
}
pub type tcpa_infect_future_t = tcpa_infect_future;

#[repr(C)]
pub struct tcpa_get_output_future {
    pub padding: [c_char; TCPA_GET_OUTPUT_FUTURE_SIZE],
}
pub type tcpa_get_output_future_t = tcpa_get_output_future;