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
