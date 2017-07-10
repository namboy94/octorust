#ifndef _OCTO_SIGNAL_H_
#define _OCTO_SIGNAL_H_

#include "octo_ilet.h"

#include "octo_signal_types.h"

/**
 * \defgroup sync_grp Synchronisation of i-lets and data
 * @{
 */

/** \file octo_signal.h
 * \brief Functions to deal with various signal data types
 * A Signal is one of the basic synchronisation construct in OctoPOS.
 *
 * With the help of a Signal it is possible to wait for a given number of
 * events. Depending on the Signal variant the current i-let can block until the
 * events arrived or spawn a new i-let on arrival.
 *
 * Reviewed by Jens @ 11.02.2013
 */

/**
 * @struct simple_signal
 * Opaque type for simple_signal.
 *
 * This type is used to allocate simple_signal "objects" with the right size.
 * To initialise a variable of this type, see function simple_signal_init().
 */
struct simple_signal {
	char padding[SIMPLE_SIGNAL_SIZE];
}__attribute__((aligned(SIMPLE_SIGNAL_ALIGNMENT)));
typedef struct simple_signal simple_signal;

/**
 * @struct infect_signal
 * Opaque type for infect_signal.
 *
 * This type is used to allocate infect_signal "objects" with the right size.
 * To initialise a variable of this type, see function infect_signal_init().
 */
struct infect_signal {
	char padding[INFECT_SIGNAL_SIZE];
}__attribute__((aligned(INFECT_SIGNAL_ALIGNMENT)));
typedef struct infect_signal infect_signal;

/**
 * @struct binary_signal
 * Opaque type for binary_signal.
 *
 * This type is used to allocate binary_signal "objects" with the right size.
 * To initialise a variable of this type, see function binary_signal_init().
 */
struct binary_signal {
	char padding[BINARY_SIGNAL_SIZE];
}__attribute__((aligned(BINARY_SIGNAL_ALIGNMENT)));
typedef struct binary_signal binary_signal;

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Initialises a given simple signal datastructure.
 *
 * @param instance Pointer to an existing SimpleSignal instance
 * @param wait_for_ilet_count Number of i-lets to wait for
 * @return 0 on success, -1 if wait_for_ilet_count is too large.
 */
int simple_signal_init(simple_signal* instance, uintptr_t wait_for_ilet_count);


/**
 * The maximum number of events waited for depends on the current OctoPOS
 * configuration. This function gives the ability determine this limit at
 * runtime.
 *
 * @return Maximum number of events possible with current OctoPOS configuration
 */
uintptr_t simple_signal_get_max_proc_count(void);


/**
 * Increase the number of waited for events.
 *
 * @param instance Pointer to an existing SimpleSignal instance
 * @param signaler_count Number of events
 * @return 0 on success, -1 if no new signalers can be added.
 */
int simple_signal_add_signalers(simple_signal* instance,
                                uintptr_t signaler_count);


/**
 * \brief Get the number of simple_signal_signal calls that are still outstanding.
 * ***WARNING***: This is unsynchronised and is basically outdated directly
 * after reading it -> use with care!
 *
 * \param instance Pointer to an existing SimpleSignal instance
 * \return value of the signal's counter
 */
uintptr_t simple_signal_get_counter(simple_signal* instance);

/**
 * Passively wait for all events to arrive.
 *
 * If all events waited for already arrived the function will return instantly.
 *
 * @param instance Pointer to an existing SimpleSignal instance
 * @note Only a single control flow can be blocked on the same signal at a time.
 *       If multiple control flows call this function simultaneously, undefined
 *       behaviour will occur.
 */
void simple_signal_wait(simple_signal* instance);

/**
 * Signal the arrival of an event.
 *
 * When the last event is signaled, the function simple_signal_wait() will
 * return.
 *
 * @param instance Pointer to an existing SimpleSignal instance
 * @note If the signal operation is the last operation of the calling i-let, it
 *       is preferable to use simple_signal_signal_and_exit() instead.
 */
void simple_signal_signal(simple_signal* instance);

/**
 * Signal the arrival of an event and exit the current i-let.
 *
 * When the last event is signaled, the function simple_signal_wait() will
 * return.
 *
 * @param instance Pointer to an existing SimpleSignal instance
 * @note This function will perform a more efficient wakeup than
 *       simple_signal_signal() by doing a direct context switch to the woken
 *       i-let (provided that i-let belongs to the same claim as the calling
 *       i-let).
 */
void simple_signal_signal_and_exit(simple_signal* instance)
__attribute__ ((noreturn));

/**
 * Initialises a given infect signal datastructure.
 *
 * @param instance Pointer to an existing InfectSignal instance
 * @param counter Number of events until i-let is started
 * @param ilet i-let started after given number of events. The i-let data
 * structure is copied and not needed anymore.
 */
void infect_signal_init(infect_signal* instance, uintptr_t counter,
                        const simple_ilet* ilet);


/**
 * Signal the arrival of an event.
 *
 * When the last event is signaled, the i-let provided to infect_signal_init()
 * is scheduled.
 *
 * @param instance Pointer to an existing InfectSignal instance
 */
void infect_signal_signal(infect_signal* instance);

/**
 * Signal the arrival of an event and exit the current i-let.
 *
 * When the last event is signaled, the i-let provided to infect_signal_init()
 * is scheduled.
 *
 * @param instance Pointer to an existing InfectSignal instance
 */
void infect_signal_signal_and_exit(infect_signal* instance)
__attribute__ ((noreturn));

/**
 * Initialises a given binary_signal datastructure.
 *
 * @param instance Pointer to an existing binary_signal instance
 */
void binary_signal_init(binary_signal* instance);

/**
 * \brief Get the number of binary_signal_signal calls that are still outstanding.
 * ***WARNING***: This is unsynchronised and is basically outdated directly
 * after reading it -> use with care!
 *
 * \param instance Pointer to an existing binary_signal instance
 * \return value of the signal's counter
 */
uintptr_t binary_signal_get_counter(binary_signal* instance);

/**
 * Passively wait for all events to arrive.
 *
 * If all events waited for already arrived the function will return instantly.
 *
 * @param instance Pointer to an existing binary_signal instance
 * @note Only a single control flow can be blocked on the same signal at a time.
 *       If multiple control flows call this function simultaneously, undefined
 *       behaviour will occur.
 */
void binary_signal_wait(binary_signal* instance);

/**
 * Signal the arrival of an event.
 *
 * When the last event is signaled, the function binary_signal_wait() will
 * return.
 *
 * @param instance Pointer to an existing binary_signal instance
 * @note If the signal operation is the last operation of the calling i-let, it
 *       is preferable to use binary_signal_signal_and_exit() instead.
 */
void binary_signal_signal(binary_signal* instance);

/**
 * Signal the arrival of an event and exit the current i-let.
 *
 * When the last event is signaled, the function binary_signal_wait() will
 * return.
 *
 * @param instance Pointer to an existing binary_signal instance
 * @note This function will perform a more efficient wakeup than
 *       binary_signal_signal() by doing a direct context switch to the woken
 *       i-let (provided that i-let belongs to the same claim as the calling
 *       i-let).
 */
void binary_signal_signal_and_exit(binary_signal* instance)
__attribute__ ((noreturn));

/** @} */ // End defgroup sync_grp

#ifdef __cplusplus
}
#endif

#endif
