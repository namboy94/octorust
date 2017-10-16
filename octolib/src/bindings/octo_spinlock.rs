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
