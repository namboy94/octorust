/**
 * \brief Stops execution on all cores on this tile except for the core on which
 *        this function is called.
 * \return 0 on success, -1 if this tile is already stopped.
 */
int gc_stop_world(void);

/**
 * \brief Resumes execution on the cores on this tile that were stopped.
 * \return 0 on success, -1 if this tile is not currently stopped.
 */
int gc_start_world(void);

/**
 * \brief Invokes a callback function for every active stack.
 * \param callback Callback function. Receives pointers to the lower and the
 *                 upper end of the respective stack.
 * \return 0 on success, -1 if this tile is not currently stopped.
 */
int gc_iterate_all_stacks(void (*callback)(const void *, const void *));

/**
 * \brief This is just for debugging purposes!
 */
void gc_stop_world_on_tile(tile_id_t tid);
