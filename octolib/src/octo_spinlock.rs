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

use octo_types::*;

extern {
    #[link_name="simple_spinlock_init"]
    fn __simple_spinlock_init(instance: *simple_spinlock);

    #[link_name="simple_spinlock_lock"]
    fn __simple_spinlock_lock(instance: *simple_spinlock);

    #[link_name="simple_spinlock_unlock"]
    fn __simple_spinlock_unlock(instance: *simple_spinlock);

    #[link_name="simple_spinlock_trylock"]
    fn __simple_spinlock_trylock(instance: *simple_spinlock) -> i32;

    #[link_name="simple_spinlock_isLocked"]
    fn __simple_spinlock_isLocked(instance: *simple_spinlock) -> i32;

    #[link_name="recursive_spinlock_init"]
    fn __recursive_spinlock_init(instance: *recursive_spinlock);

    #[link_name="recursive_spinlock_lock"]
    fn __recursive_spinlock_lock(instance: *recursive_spinlock);

    #[link_name="recursive_spinlock_trylock"]
    fn __recursive_spinlock_trylock(instance: *recursive_spinlock) -> i32;

    #[link_name="recursive_spinlock_isLocked"]
    fn __recursive_spinlock_isLocked(instance: *recursive_spinlock) -> i32;

    #[link_name="recursive_spinlock_unlock"]
    fn __recursive_spinlock_unlock(instance: *recursive_spinlock);

}

pub fn simple_spinlock_init(instance: *simple_spinlock) {
    unsafe {
        __simple_spinlock_init(instance)
    }
}

pub fn simple_spinlock_lock(instance: *simple_spinlock) {
    unsafe {
        __simple_spinlock_lock(instance)
    }
}

pub fn simple_spinlock_unlock(instance: *simple_spinlock) {
    unsafe {
        __simple_spinlock_unlock(instance)
    }
}

pub fn simple_spinlock_trylock(instance: *simple_spinlock) -> i32 {
    unsafe {
        __simple_spinlock_trylock(instance)
    }
}

pub fn simple_spinlock_isLocked(instance: *simple_spinlock) -> i32 {
    unsafe {
        __simple_spinlock_isLocked(instance)
    }
}

pub fn recursive_spinlock_init(instance: *recursive_spinlock) {
    unsafe {
        __recursive_spinlock_init(instance)
    }
}

pub fn recursive_spinlock_lock(instance: *recursive_spinlock) {
    unsafe {
        __recursive_spinlock_lock(instance)
    }
}

pub fn recursive_spinlock_trylock(instance: *recursive_spinlock) -> i32 {
    unsafe {
        __recursive_spinlock_trylock(instance)
    }
}

pub fn recursive_spinlock_isLocked(instance: *recursive_spinlock) -> i32 {
    unsafe {
        __recursive_spinlock_isLocked(instance)
    }
}

pub fn recursive_spinlock_unlock(instance: *recursive_spinlock) {
    unsafe {
        __recursive_spinlock_unlock(instance)
    }
}

