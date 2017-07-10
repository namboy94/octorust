#ifndef _OCTO_MEMORY_H_
#define _OCTO_MEMORY_H_

#include <stdint.h>
#include "cfAttribs.h"

/**
 * \file octo_memory.h
 *
 * \brief Functions for dynamic memory allocation
 */

#ifdef __cplusplus
extern "C" {
#endif

/** \brief Memory types. */
enum {
	MEM_TLM_LOCAL = 0, /**< Tile-local memory from this tile's local address space. */
	MEM_TLM_GLOBAL = 1, /**< Tile-local memory somewhere in the range of the tile's shared address space. */
	MEM_SHM = 2,  /**< Shared (global) memory. */
	MEM_ICM = 3, /**< iCore memory. */
	MEM_TYPES_SIZE = 4,  /* this is the number of valid MEM_types*/
	MEM_INVALID = -1  /**< Invalid memory region. */
};

/**
 * OCTO_LIBC_MEM_TYPE defines the memory type for malloc, realloc, free,
 * posix_memalign, aligned_alloc and memalign
 */ 
#if defined(cf_OCTO_LIBC_ALLOC_TLM)
	#define OCTO_LIBC_MEM_TYPE MEM_TLM_LOCAL
#elif defined(cf_OCTO_LIBC_ALLOC_SHM)
	#define OCTO_LIBC_MEM_TYPE MEM_SHM
#else
	#define OCTO_LIBC_MEM_TYPE MEM_TLM_LOCAL
#endif

/**
 * \brief Allocates uninitialised memory, which is suitably aligned for any
 *        built-in type and DMAs.
 * \param type Memory type.
 * \param size Size of the chunk to be allocated (in bytes).
 * \return Pointer to the allocated chunk on success, NULL on error.
 */
void *mem_allocate(int type, uintptr_t size)
	__attribute__((warn_unused_result, malloc));

/**
 * \brief Allocates uninitialised memory with at least the given alignment.
 * \param type      Memory type.
 * \param alignment Alignment of the allocated memory.
 * \param size      Size of the chunk to be allocated (in bytes). This must be a
 *                  power of two.
 * \return Pointer to the allocated memory on success, NULL on error.
 */
void *mem_allocate_aligned(int type, uintptr_t alignment, uintptr_t size)
	__attribute__((warn_unused_result, malloc));

/**
 * \brief Resizes memory, preserving its contents but not
 *        necessarily its address. If new_size is equal to zero, and p is not
 *        NULL, then the call is equivalent to mem_free(p) If p is NULL,
 *        mem_reallocate() behaves like mem_allocate(). Otherwise, if p does
 *        not point to a previously allocated chunk, NULL is returned.
 * \param type     Memory type. This must be the same as the type of the
 *                 original chunk, otherwise no action is performed and NULL is
 *                 returned.
 * \param p        Pointer to the chunk to be resized.
 * \param new_size New size of the chunk (in bytes).
 * \return Pointer to the resized chunk on success, NULL on error. In the
 *         latter case, the original chunk is left untouched.
 */
void *mem_reallocate(int type, void *p, uintptr_t new_size)
	__attribute__((warn_unused_result));

/**
 * \brief Releases an allocated chunk of memory.
 * \param p Pointer to the memory chunk to be released. If p is NULL, no
 *          action is performed. Otherwise, if it does not point to a
 *          previously allocated chunk, a trap is triggered.
 */
void mem_free(void *p);



/** \brief Determines the memory type for a given address. */
int mem_get_type(const void *p);

/** Returns the page size of the system. */
uintptr_t mem_get_page_size(void) __attribute__((const));

/**
 * \brief Returns the current number of free pages managed for a given memory
 * type.
 * \param type Memory type.
 * \return Number of pages managed, or -1 if type is invalid.
 */
intptr_t mem_get_total_page_count(int type);

/**
 * \brief Allocates contiguous memory pages.
 * \param type Memory type.
 * \param size Number of bytes to be allocated. This must be a non-zero
 *        multiple of the page size.
 * \return Pointer to the allocated memory on success, NULL on error. This
 *         pointer will be aligned at a page boundary.
 */
void *mem_map(int type, uintptr_t size)
	__attribute__((warn_unused_result, malloc));

/**
 * \brief Allocates memory pages with at least the given alignment.
 * \param type Memory type.
 * \param alignment Alignment of the allocated memory.
 * \param size Number of bytes to be allocated. This must be a non-zero
 *        multiple of the page size.
 * \return Pointer to the allocated memory on success, NULL on error.
 */
void *mem_map_aligned(int type, uintptr_t alignment, uintptr_t size)
	__attribute__((warn_unused_result, malloc));

/**
 * \brief If possible, allocates memory pages after the region specified by the
 *        start address 'p' with the given size 'size', resulting in a memory
 *        region with start address 'p' and the new size 'newsize'. 'size' and
 *        'newsize' must be a non-zero multiple of page size, 'newsize' must be
 *        larger than 'size' and 'p' must point to a valid memory region that
 *        was previously allocated. With this function efficient
 *        implementations of realloc-like functions are possible.
 * \param p Start address of the memory region to be grown
 * \param size Number of bytes that belong to the region starting with 'p'
 * \param newsize Number of bytes for the new memory region starting at 'p'.
 * \return 0 on success, -1 if the memory region could not be expanded or
 *         any parameter was invalid
 */
int mem_map_grow(void *p, uintptr_t size, uintptr_t newsize)
	__attribute__((warn_unused_result));

/**
 * \brief Releases a number of contiguous allocated memory pages.
 * \param p    Pointer to the beginning of the first page to be released. If p
 *             does not point to a previously allocated page, no action is
 *             performed and -1 is returned.
 * \param size Number of bytes to be released. If this is not a multiple of the
 *             page size, no action is performed and -1 is returned.
 * \return     0 on success, -1 on error.
 */
int mem_unmap(void *p, uintptr_t size);

/**
 * \brief Safely allocate memory within the local TLM range.
 * 
 * \param size  The number of bytes to allocate.
 * \return      A pointer to the allocated memory.
 */
void* safe_malloc(uintptr_t size);

#ifdef __cplusplus
}
#endif

#endif /* _OCTO_MEMORY_H_ */
