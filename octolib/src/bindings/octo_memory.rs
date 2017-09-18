/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_memory.h file

/// Functions for dynamic memory allocation

use octo_types::*;

extern {
    #[link_name="*mem_allocate"]
    fn __mem_allocate(_type: i32, size: usize) -> *mut c_void;

    #[link_name="*mem_allocate_aligned"]
    fn __mem_allocate_aligned(_type: i32, alignment: usize, size: usize) -> *mut c_void;

    #[link_name="*mem_reallocate"]
    fn __mem_reallocate(_type: i32, p: *mut c_void, new_size: usize) -> *mut c_void;

    #[link_name="mem_free"]
    fn __mem_free(p: *mut c_void);

    #[link_name="mem_get_type"]
    fn __mem_get_type(p: *const c_void) -> i32;

    #[link_name="mem_get_page_size"]
    fn __mem_get_page_size() -> usize;

    #[link_name="mem_get_total_page_count"]
    fn __mem_get_total_page_count(_type: i32) -> isize;

    #[link_name="*mem_map"]
    fn __mem_map(_type: i32, size: usize) -> *mut c_void;

    #[link_name="*mem_map_aligned"]
    fn __mem_map_aligned(_type: i32, alignment: usize, size: usize) -> *mut c_void;

    #[link_name="mem_map_grow"]
    fn __mem_map_grow(p: *mut c_void, size: usize, newsize: usize) -> i32;

    #[link_name="mem_unmap"]
    fn __mem_unmap(p: *mut c_void, size: usize) -> i32;

    #[link_name="safe_malloc"]
    fn __safe_malloc(size: usize) -> *mut c_void;

}

/// Allocates uninitialised memory, which is suitably aligned for any
/// built-in type and DMAs.
///
/// # Arguments
///
/// * `type` - Memory type.
/// * `size` - Size of the chunk to be allocated (in bytes).
///
/// # Return Value
///
/// Pointer to the allocated chunk on success, NULL on error.
pub fn mem_allocate(_type: i32, size: usize) -> *mut c_void {
    unsafe {
        __mem_allocate(_type, size)
    }
}

/// Allocates uninitialised memory with at least the given alignment.
///
/// # Arguments
///
/// * `type` -      Memory type.
/// * `alignment` - Alignment of the allocated memory.
/// * `size` -      Size of the chunk to be allocated (in bytes). This must be a
///                 power of two.
/// # Return Value
///
/// Pointer to the allocated memory on success, NULL on error.
pub fn mem_allocate_aligned(_type: i32, alignment: usize, size: usize) -> *mut c_void {
    unsafe {
        __mem_allocate_aligned(_type, alignment, size)
    }
}

/// Resizes memory, preserving its contents but not
///        necessarily its address. If new_size is equal to zero, and p is not
///        NULL, then the call is equivalent to mem_free(p) If p is NULL,
///        mem_reallocate() behaves like mem_allocate(). Otherwise, if p does
///        not point to a previously allocated chunk, NULL is returned.
///
/// # Arguments
///
/// * `type` -     Memory type. This must be the same as the type of the
///                original chunk, otherwise no action is performed and NULL is
///                returned.
/// * `p` -        Pointer to the chunk to be resized.
/// * `new_size` - New size of the chunk (in bytes).
///
/// # Return Value
///
/// Pointer to the resized chunk on success, NULL on error. In the
/// latter case, the original chunk is left untouched.
pub fn mem_reallocate(_type: i32, p: *mut c_void, new_size: usize) -> *mut c_void {
    unsafe {
        __mem_reallocate(_type, p, new_size)
    }
}

/// Releases an allocated chunk of memory.
///
/// # Arguments
///
/// * `p` - Pointer to the memory chunk to be released. If p is NULL, no
///         action is performed. Otherwise, if it does not point to a
///         previously allocated chunk, a trap is triggered.
pub fn mem_free(p: *mut c_void) {
    unsafe {
        __mem_free(p)
    }
}

/// Determines the memory type for a given address.///
pub fn mem_get_type(p: *const c_void) -> i32 {
    unsafe {
        __mem_get_type(p)
    }
}

/// Returns the page size of the system.///
pub fn mem_get_page_size() -> usize {
    unsafe {
        __mem_get_page_size()
    }
}

/// Returns the current number of free pages managed for a given memory
/// type.
///
/// # Arguments
///
/// * `type` - Memory type.
///
/// # Return Value
///
/// Number of pages managed, or -1 if type is invalid.
pub fn mem_get_total_page_count(_type: i32) -> isize {
    unsafe {
        __mem_get_total_page_count(_type)
    }
}

/// Allocates contiguous memory pages.
///
/// # Arguments
///
/// * `type` - Memory type.
/// * `size` - Number of bytes to be allocated. This must be a non-zero
///            multiple of the page size.
/// # Return Value
///
/// Pointer to the allocated memory on success, NULL on error. This
/// pointer will be aligned at a page boundary.
pub fn mem_map(_type: i32, size: usize) -> *mut c_void {
    unsafe {
        __mem_map(_type, size)
    }
}

/// Allocates memory pages with at least the given alignment.
///
/// # Arguments
///
/// * `type` - Memory type.
/// * `alignment` - Alignment of the allocated memory.
/// * `size` - Number of bytes to be allocated. This must be a non-zero
///            multiple of the page size.
///
/// # Return Value
///
/// Pointer to the allocated memory on success, NULL on error.
pub fn mem_map_aligned(_type: i32, alignment: usize, size: usize) -> *mut c_void {
    unsafe {
        __mem_map_aligned(_type, alignment, size)
    }
}

/// If possible, allocates memory pages after the region specified by the
/// start address 'p' with the given size 'size', resulting in a memory
/// region with start address 'p' and the new size 'newsize'. 'size' and
/// 'newsize' must be a non-zero multiple of page size, 'newsize' must be
/// larger than 'size' and 'p' must point to a valid memory region that
/// was previously allocated. With this function efficient
/// implementations of realloc-like functions are possible.
///
/// # Arguments
///
/// * `p` - Start address of the memory region to be grown
/// * `size` - Number of bytes that belong to the region starting with 'p'
/// * `newsize` - Number of bytes for the new memory region starting at 'p'.
///
/// # Return Value
///
/// 0 on success, -1 if the memory region could not be expanded or
/// any parameter was invalid
pub fn mem_map_grow(p: *mut c_void, size: usize, newsize: usize) -> i32 {
    unsafe {
        __mem_map_grow(p, size, newsize)
    }
}

/// Releases a number of contiguous allocated memory pages.
///
/// # Arguments
///
/// * `p` -    Pointer to the beginning of the first page to be released. If p
///            does not point to a previously allocated page, no action is
///            performed and -1 is returned.
/// * `size` - Number of bytes to be released. If this is not a multiple of the
///            page size, no action is performed and -1 is returned.
///
/// # Return Value
///
///  0 on success, -1 on error.
pub fn mem_unmap(p: *mut c_void, size: usize) -> i32 {
    unsafe {
        __mem_unmap(p, size)
    }
}

/// Safely allocate memory within the local TLM range.
///
/// # Arguments
///
/// * `size` -  The number of bytes to allocate.
///
/// # Return Value
///
///  A pointer to the allocated memory.
pub fn safe_malloc(size: usize) -> *mut c_void {
    unsafe {
        __safe_malloc(size)
    }
}
