/// Functions for dynamic memory allocation

/// Memory types.
enum MEMTYPES {
	MEM_TLM_LOCAL = 0, /**< Tile-local memory from this tile's local address space. */
	MEM_TLM_GLOBAL = 1, /**< Tile-local memory somewhere in the range of the tile's shared address space. */
	MEM_SHM = 2,  /**< Shared (global) memory. */
	MEM_ICM = 3, /**< iCore memory. */
	MEM_TYPES_SIZE = 4,  /* this is the number of valid MEM_types*/
	MEM_INVALID = -1  /**< Invalid memory region. */
}