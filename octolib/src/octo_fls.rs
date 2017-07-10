/// Fibre-local-storage functions.
///
/// "Fibre" is another name for an i-let in execution. Fibre-local storage
/// enables the programmer to store small amounts of data (usually pointers) in a
/// memory area whose lifespan is identical to the lifespan of the fibre. The
/// values are preserved across blocking operations.
///
/// The programming interface is very similar to the Windows API.
///
/// Fibre-local storage supersedes the old "i-let-local data" interface, which
/// has imprecise semantics and is awkward to use by more than one program
/// component.

use octo_types::*;

extern {
    // TODO Figure out how to port this
    // Original: intptr_t fls_allocate(void (*destructor)(void *));
    //#[link_name="fls_allocate"]
    //fn __fls_allocate(*): void (*destructor)(void) -> intptr_t;

    #[link_name="fls_free"]
    fn __fls_free(index: usize) -> i32;

    #[link_name="fls_set"]
    fn __fls_set(index: usize, value: *mut c_void) -> i32;

    #[link_name="fls_get"]
    fn __fls_get(index: usize) -> *mut c_void;

}

/*
/// Allocates a fibre-local-storage index.
/// Any fibre on this tile can subsequently use the returned index to store and
/// retrieve values that are local to the fibre. If the allocation fails, -1 is
/// returned.
///
/// # Arguments
///
/// * `destructor` - An optional pointer to a callback function. If the
///                  associated slot is in use (i.e. non-NULL), this function will be
///                  called in the following situations:
///                  - A fibre finishes execution.
///                  - A fibre-local-storage index is freed.
pub fn fls_allocate(*): void (*destructor)(void) -> intptr_t {
    unsafe {
        __fls_allocate(*))
    }
}
*/

/// Releases a fibre-local-storage index, making it available for reuse on
/// this tile.
///
/// # Return Value
///
/// 0 on success, -1 on error.
pub fn fls_free(index: usize) -> i32 {
    unsafe {
        __fls_free(index)
    }
}

/// Stores a pointer value in the calling i-let's fibre-local-storage slot
/// for the specified index.
///
/// # Return Value
///
/// 0 on success, -1 on error.
pub fn fls_set(index: usize, value: *mut c_void) -> i32 {
    unsafe {
        __fls_set(index, value)
    }
}

/// Retrieves a pointer value previously stored in the calling i-let's
/// fibre-local-storage slot for the specified index. If the slot is
/// uninitialised, NULL is returned.
pub fn fls_get(index: usize) -> *mut c_void {
    unsafe {
        __fls_get(index)
    }
}

