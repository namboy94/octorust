/// Operations on spinlocks

use octo_structs::*;

extern {
    #[link_name="simple_spinlock_init"]
    fn __simple_spinlock_init(instance: *mut simple_spinlock);

    #[link_name="simple_spinlock_lock"]
    fn __simple_spinlock_lock(instance: *mut simple_spinlock);

    #[link_name="simple_spinlock_unlock"]
    fn __simple_spinlock_unlock(instance: *mut simple_spinlock);

    #[link_name="simple_spinlock_trylock"]
    fn __simple_spinlock_trylock(instance: *mut simple_spinlock) -> i32;

    #[link_name="simple_spinlock_isLocked"]
    fn __simple_spinlock_isLocked(instance: *mut simple_spinlock) -> i32;

    #[link_name="recursive_spinlock_init"]
    fn __recursive_spinlock_init(instance: *mut recursive_spinlock);

    #[link_name="recursive_spinlock_lock"]
    fn __recursive_spinlock_lock(instance: *mut recursive_spinlock);

    #[link_name="recursive_spinlock_trylock"]
    fn __recursive_spinlock_trylock(instance: *mut recursive_spinlock) -> i32;

    #[link_name="recursive_spinlock_isLocked"]
    fn __recursive_spinlock_isLocked(instance: *mut recursive_spinlock) -> i32;

    #[link_name="recursive_spinlock_unlock"]
    fn __recursive_spinlock_unlock(instance: *mut recursive_spinlock);

}

pub fn simple_spinlock_init(instance: *mut simple_spinlock) {
    unsafe {
        __simple_spinlock_init(instance)
    }
}

pub fn simple_spinlock_lock(instance: *mut simple_spinlock) {
    unsafe {
        __simple_spinlock_lock(instance)
    }
}

pub fn simple_spinlock_unlock(instance: *mut simple_spinlock) {
    unsafe {
        __simple_spinlock_unlock(instance)
    }
}

pub fn simple_spinlock_trylock(instance: *mut simple_spinlock) -> i32 {
    unsafe {
        __simple_spinlock_trylock(instance)
    }
}

pub fn simple_spinlock_isLocked(instance: *mut simple_spinlock) -> i32 {
    unsafe {
        __simple_spinlock_isLocked(instance)
    }
}

pub fn recursive_spinlock_init(instance: *mut recursive_spinlock) {
    unsafe {
        __recursive_spinlock_init(instance)
    }
}

pub fn recursive_spinlock_lock(instance: *mut recursive_spinlock) {
    unsafe {
        __recursive_spinlock_lock(instance)
    }
}

pub fn recursive_spinlock_trylock(instance: *mut recursive_spinlock) -> i32 {
    unsafe {
        __recursive_spinlock_trylock(instance)
    }
}

pub fn recursive_spinlock_isLocked(instance: *mut recursive_spinlock) -> i32 {
    unsafe {
        __recursive_spinlock_isLocked(instance)
    }
}

pub fn recursive_spinlock_unlock(instance: *mut recursive_spinlock) {
    unsafe {
        __recursive_spinlock_unlock(instance)
    }
}

