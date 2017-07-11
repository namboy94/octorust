/// Functions to deal with various signal data types
/// A Signal is one of the basic synchronisation construct in OctoPOS.
///
/// With the help of a Signal it is possible to wait for a given number of
/// events. Depending on the Signal variant the current i-let can block until the
/// events arrived or spawn a new i-let on arrival.

use constants::*;
use octo_types::*;
use octo_ilet::*;

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

// TODO Implement these functions:

/*
/// Signal the arrival of an event and exit the current i-let.
///
/// When the last event is signaled, the function binary_signal_wait() will
/// return.
///
/// @param instance Pointer to an existing binary_signal instance
/// @note This function will perform a more efficient wakeup than
///       binary_signal_signal() by doing a direct context switch to the woken
///       i-let (provided that i-let belongs to the same claim as the calling
///       i-let).
void binary_signal_signal_and_exit(binary_signal* instance)
__attribute__ ((noreturn));

/// Signal the arrival of an event and exit the current i-let.
///
/// When the last event is signaled, the function simple_signal_wait() will
/// return.
///
/// @param instance Pointer to an existing SimpleSignal instance
/// @note This function will perform a more efficient wakeup than
///       simple_signal_signal() by doing a direct context switch to the woken
///       i-let (provided that i-let belongs to the same claim as the calling
///       i-let).
void simple_signal_signal_and_exit(simple_signal* instance)
__attribute__ ((noreturn));

/// Signal the arrival of an event and exit the current i-let.
///
/// When the last event is signaled, the i-let provided to infect_signal_init()
/// is scheduled.
///
/// @param instance Pointer to an existing InfectSignal instance
void infect_signal_signal_and_exit(infect_signal* instance)
__attribute__ ((noreturn));
*/

extern {
    #[link_name="simple_signal_init"]
    fn __simple_signal_init(instance: *mut simple_signal, wait_for_ilet_count: usize) -> i32;

    #[link_name="simple_signal_get_max_proc_count"]
    fn __simple_signal_get_max_proc_count() -> usize;

    #[link_name="simple_signal_add_signalers"]
    fn __simple_signal_add_signalers(instance: *mut simple_signal, signaler_count: usize) -> i32;

    #[link_name="simple_signal_get_counter"]
    fn __simple_signal_get_counter(instance: *mut simple_signal) -> usize;

    #[link_name="simple_signal_wait"]
    fn __simple_signal_wait(instance: *mut simple_signal);

    #[link_name="simple_signal_signal"]
    fn __simple_signal_signal(instance: *mut simple_signal);

    #[link_name="infect_signal_init"]
    fn __infect_signal_init(instance: *mut infect_signal, counter: usize, ilet: *const simple_ilet);

    #[link_name="infect_signal_signal"]
    fn __infect_signal_signal(instance: *mut infect_signal);

    #[link_name="binary_signal_init"]
    fn __binary_signal_init(instance: *mut binary_signal);

    #[link_name="binary_signal_get_counter"]
    fn __binary_signal_get_counter(instance: *mut binary_signal) -> usize;

    #[link_name="binary_signal_wait"]
    fn __binary_signal_wait(instance: *mut binary_signal);

    #[link_name="binary_signal_signal"]
    fn __binary_signal_signal(instance: *mut binary_signal);

}

/// Initialises a given simple signal datastructure.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing SimpleSignal instance
/// * `wait_for_ilet_count` - Number of i-lets to wait for
///
/// # Return Value
///
/// 0 on success, -1 if wait_for_ilet_count is too large.
pub fn simple_signal_init(instance: *mut simple_signal, wait_for_ilet_count: usize) -> i32 {
    unsafe {
        __simple_signal_init(instance, wait_for_ilet_count)
    }
}

/// The maximum number of events waited for depends on the current OctoPOS
/// configuration. This function gives the ability determine this limit at
/// runtime.
///
/// # Return Value
///
/// Maximum number of events possible with current OctoPOS configuration
pub fn simple_signal_get_max_proc_count() -> usize {
    unsafe {
        __simple_signal_get_max_proc_count()
    }
}

/// Increase the number of waited for events.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing SimpleSignal instance
/// * `signaler_count` - Number of events
///
/// # Return Value
///
/// 0 on success, -1 if no new signalers can be added.
pub fn simple_signal_add_signalers(instance: *mut simple_signal, signaler_count: usize) -> i32 {
    unsafe {
        __simple_signal_add_signalers(instance, signaler_count)
    }
}

/// Get the number of simple_signal_signal calls that are still outstanding.
/// **WARNING**: This is unsynchronised and is basically outdated directly
/// after reading it -> use with care!
///
/// # Arguments
///
/// * `instance` - Pointer to an existing SimpleSignal instance
///
/// # Return Value
///
/// value of the signal's counter
pub fn simple_signal_get_counter(instance: *mut simple_signal) -> usize {
    unsafe {
        __simple_signal_get_counter(instance)
    }
}

/// Passively wait for all events to arrive.
/// If all events waited for already arrived the function will return instantly.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing SimpleSignal instance
///
/// # Note
///
///  Only a single control flow can be blocked on the same signal at a time.
///  If multiple control flows call this function simultaneously, undefined
///  behaviour will occur.
pub fn simple_signal_wait(instance: *mut simple_signal) {
    unsafe {
        __simple_signal_wait(instance)
    }
}

/// Signal the arrival of an event.
/// When the last event is signaled, the function simple_signal_wait() will
/// return.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing SimpleSignal instance
///
///
/// # Note
///
///  If the signal operation is the last operation of the calling i-let, it
///  is preferable to use simple_signal_signal_and_exit() instead.
pub fn simple_signal_signal(instance: *mut simple_signal) {
    unsafe {
        __simple_signal_signal(instance)
    }
}

/// Initialises a given infect signal datastructure.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing InfectSignal instance
/// * `counter` - Number of events until i-let is started
/// * `ilet` - i-let started after given number of events. The i-let data
///
/// structure is copied and not needed anymore.
pub fn infect_signal_init(instance: *mut infect_signal, counter: usize, ilet: *const simple_ilet) {
    unsafe {
        __infect_signal_init(instance, counter, ilet)
    }
}

/// Signal the arrival of an event.
/// When the last event is signaled, the i-let provided to infect_signal_init()
/// is scheduled.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing InfectSignal instance
pub fn infect_signal_signal(instance: *mut infect_signal) {
    unsafe {
        __infect_signal_signal(instance)
    }
}

/// Initialises a given binary_signal datastructure.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing binary_signal instance
pub fn binary_signal_init(instance: *mut binary_signal) {
    unsafe {
        __binary_signal_init(instance)
    }
}

/// Get the number of binary_signal_signal calls that are still outstanding.
/// **WARNING**: This is unsynchronised and is basically outdated directly
/// after reading it -> use with care!
///
/// # Arguments
///
/// * `instance` - Pointer to an existing binary_signal instance
///
/// # Return Value
///
/// value of the signal's counter
///
pub fn binary_signal_get_counter(instance: *mut binary_signal) -> usize {
    unsafe {
        __binary_signal_get_counter(instance)
    }
}

/// Passively wait for all events to arrive.
/// If all events waited for already arrived the function will return instantly.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing binary_signal instance
///
///
/// # Note
///
///  Only a single control flow can be blocked on the same signal at a time.
///  If multiple control flows call this function simultaneously, undefined
///  behaviour will occur.
pub fn binary_signal_wait(instance: *mut binary_signal) {
    unsafe {
        __binary_signal_wait(instance)
    }
}

/// Signal the arrival of an event.
/// When the last event is signaled, the function binary_signal_wait() will
/// return.
///
/// # Arguments
///
/// * `instance` - Pointer to an existing binary_signal instance
///
///
/// # Note
///
///  If the signal operation is the last operation of the calling i-let, it
///  is preferable to use binary_signal_signal_and_exit() instead.
pub fn binary_signal_signal(instance: *mut binary_signal) {
    unsafe {
        __binary_signal_signal(instance)
    }
}
