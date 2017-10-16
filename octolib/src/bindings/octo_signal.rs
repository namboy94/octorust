/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_signal.h file
///
/// Functions to deal with various signal data types
/// A Signal is one of the basic synchronisation construct in OctoPOS.
///
/// With the help of a Signal it is possible to wait for a given number of
/// events. Depending on the Signal variant the current i-let can block until the
/// events arrived or spawn a new i-let on arrival.

use octo_structs::*;

extern {

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
    pub fn simple_signal_init(instance: *mut simple_signal, wait_for_ilet_count: usize) -> i32;

    /// The maximum number of events waited for depends on the current OctoPOS
    /// configuration. This function gives the ability determine this limit at
    /// runtime.
    ///
    /// # Return Value
    ///
    /// Maximum number of events possible with current OctoPOS configuration
    pub fn simple_signal_get_max_proc_count() -> usize;

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
    pub fn simple_signal_add_signalers(instance: *mut simple_signal, signaler_count: usize) -> i32;

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
    pub fn simple_signal_get_counter(instance: *mut simple_signal) -> usize;

    /// Passively wait for all events to arrive.
    /// If all events waited for already arrived the function will return instantly.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing SimpleSignal instance
    ///
    /// # Note
    ///
    /// Only a single control flow can be blocked on the same signal at a time.
    /// If multiple control flows call this function simultaneously, undefined
    /// behaviour will occur.
    pub fn simple_signal_wait(instance: *mut simple_signal);

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
    pub fn simple_signal_signal(instance: *mut simple_signal);

    /// Initialises a given infect signal datastructure.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing InfectSignal instance
    /// * `counter` - Number of events until i-let is started
    /// * `ilet` - i-let started after given number of events. The i-let data
    ///
    /// structure is copied and not needed anymore.
    pub fn infect_signal_init(instance: *mut infect_signal, counter: usize, ilet: *const simple_ilet);

    /// Signal the arrival of an event.
    /// When the last event is signaled, the i-let provided to infect_signal_init()
    /// is scheduled.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing InfectSignal instance
    pub fn infect_signal_signal(instance: *mut infect_signal);

    /// Initialises a given binary_signal datastructure.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing binary_signal instance
    pub fn binary_signal_init(instance: *mut binary_signal);

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
    pub fn binary_signal_get_counter(instance: *mut binary_signal) -> usize;

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
    pub fn binary_signal_wait(instance: *mut binary_signal);

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
    pub fn binary_signal_signal(instance: *mut binary_signal);

    /// Signal the arrival of an event and exit the current i-let.
    ///
    /// When the last event is signaled, the function binary_signal_wait() will
    /// return.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing binary_signal instance
    ///
    /// # Note
    ///
    /// This function will perform a more efficient wakeup than
    /// binary_signal_signal() by doing a direct context switch to the woken
    /// i-let (provided that i-let belongs to the same claim as the calling i-let).
    pub fn binary_signal_signal_and_exit(instance: *mut binary_signal);

    /// Signal the arrival of an event and exit the current i-let.
    ///
    /// When the last event is signaled, the function simple_signal_wait() will
    /// return.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing SimpleSignal instance
    /// # Note
    ///
    /// This function will perform a more efficient wakeup than
    /// simple_signal_signal() by doing a direct context switch to the woken
    /// i-let (provided that i-let belongs to the same claim as the calling i-let).
    pub fn simple_signal_signal_and_exit(instance: *mut simple_signal);

    /// Signal the arrival of an event and exit the current i-let.
    ///
    /// When the last event is signaled, the i-let provided to infect_signal_init()
    /// is scheduled.
    ///
    /// # Arguments
    ///
    /// * `instance` - Pointer to an existing InfectSignal instance
    pub fn infect_signal_signal_and_exit(instance: *mut infect_signal);

}
