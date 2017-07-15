/// Operations on spinlocks

use constants::*;

#[repr(C)]
pub struct simple_spinlock {
    pub padding: [c_char; SIMPLE_SPINLOCK_SIZE],
}

#[repr(C)]
pub struct recursive_spinlock {
    pub padding: [c_char; RECURSIVE_SPINLOCK_SIZE],
}
