#ifndef _OCTO_SPINLOCK_H_
#define _OCTO_SPINLOCK_H_

#include "octo_spinlock_types.h"

/**
 * \addtogroup sync_grp
 */

/**
 * \file octo_spinlock.h
 *
 * \brief Operations on spinlocks
 */

struct simple_spinlock {
	char padding[SIMPLE_SPINLOCK_SIZE];
}__attribute__((aligned(SIMPLE_SPINLOCK_ALIGNMENT)));
typedef struct simple_spinlock simple_spinlock;

struct recursive_spinlock {
	char padding[RECURSIVE_SPINLOCK_SIZE];
}__attribute__((aligned(RECURSIVE_SPINLOCK_ALIGNMENT)));
typedef struct recursive_spinlock recursive_spinlock;

#ifdef __cplusplus
extern "C" {
#endif

void simple_spinlock_init(simple_spinlock* instance);

void simple_spinlock_lock(simple_spinlock* instance);

void simple_spinlock_unlock(simple_spinlock* instance);

int simple_spinlock_trylock(simple_spinlock* instance);

int simple_spinlock_isLocked(simple_spinlock* instance);

void recursive_spinlock_init(recursive_spinlock* instance);

void recursive_spinlock_lock(recursive_spinlock* instance);

int recursive_spinlock_trylock(recursive_spinlock* instance);

int recursive_spinlock_isLocked(recursive_spinlock* instance);

void recursive_spinlock_unlock(recursive_spinlock* instance);

/** @} */ // End defgroup sync_grp

#ifdef __cplusplus
}
#endif

#endif
