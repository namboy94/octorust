/**
 * \file  octo_fls.h
 * \brief Fibre-local-storage functions.
 *
 * "Fibre" is another name for an i-let in execution. Fibre-local storage
 * enables the programmer to store small amounts of data (usually pointers) in a
 * memory area whose lifespan is identical to the lifespan of the fibre. The
 * values are preserved across blocking operations.
 *
 * The programming interface is very similar to the Windows API.
 *
 * Fibre-local storage supersedes the old "i-let-local data" interface, which
 * has imprecise semantics and is awkward to use by more than one program
 * component.
 */

#ifndef _OCTO_FLS_H_
#define _OCTO_FLS_H_

#include <inttypes.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Allocates a fibre-local-storage index.
 *
 * Any fibre on this tile can subsequently use the returned index to store and
 * retrieve values that are local to the fibre. If the allocation fails, -1 is
 * returned.
 *
 * \param destructor An optional pointer to a callback function. If the
 *         associated slot is in use (i.e. non-NULL), this function will be
 *         called in the following situations:
 *         - A fibre finishes execution.
 *         - A fibre-local-storage index is freed.
 */
intptr_t fls_allocate(void (*destructor)(void *));

/**
 * \brief Releases a fibre-local-storage index, making it available for reuse on
 *        this tile.
 * \return 0 on success, -1 on error.
 */
int fls_free(uintptr_t index);

/**
 * \brief Stores a pointer value in the calling i-let's fibre-local-storage slot
 *        for the specified index.
 * \return 0 on success, -1 on error.
 */
int fls_set(uintptr_t index, void *value);

/**
 * \brief Retrieves a pointer value previously stored in the calling i-let's
 *        fibre-local-storage slot for the specified index. If the slot is
 *        uninitialised, NULL is returned.
 */
void *fls_get(uintptr_t index);

#ifdef __cplusplus
}
#endif

#endif // _OCTO_FLS_H_
