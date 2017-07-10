// Local DMA engine.

use octo_types::*;
use octo_ilet::*;
use octo_syscall_future::*;

extern {
    #[link_name="sync_dma_transfer"]
    fn __sync_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32);

    #[link_name="async_ldma_copy"]
    fn __async_ldma_copy(dest: *mut c_void, src: *const c_void, len: u32, future: *mut syscall_future);

    #[link_name="async_ldma_copy_ilet"]
    fn __async_ldma_copy_ilet(dest: *mut c_void, src: *const c_void, len: u32, ilet: *mut simple_ilet);

    #[link_name="async_dma_transfer"]
    fn __async_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32, ilet: *mut simple_ilet);

    #[link_name="sync_dma_memset"]
    fn __sync_dma_memset(dst: *mut c_void, val: u32, len: u32);

}

/// Copy memory using the local DMA engine.
/// The sync_dma_transfer() function copies len bytes from memory area src to memory area dest.
///
/// # Arguments
///
/// * `src` - A pointer to the source buffer
/// * `dest` - A pointer to the destionation buffer
/// * `len` - the number of bytes to copy.
pub fn sync_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32) {
    unsafe {
        __sync_dma_transfer(src, dest, len)
    }
}

/// Asynchronously copy memory using the local DMA engine.
/// The async_ldma_copy() functions copies len bytes from memory area
/// src to the memorey area dest. You can await the completion of the
/// operation using the syscall_future API.
///
/// # Arguments
///
/// * `src` - A pointer to the source buffer.
/// * `dest` - A pointer to the destination buffer.
/// * `len` - the number of bytes to copy.
pub fn async_ldma_copy(dest: *mut c_void, src: *const c_void, len: u32, future: *mut syscall_future) {
    unsafe {
        __async_ldma_copy(dest, src, len, future)
    }
}

/// Asynchronously copy memory using the local DMA engine.
/// The async_dma_transfer() function copies len bytes from memory area
/// src to memory area dest and schedules ilet when the copy is
/// finished.
///
/// # Arguments
///
/// * `src` - A pointer to the source buffer
/// * `dest` - A pointer to the destionation buffer
/// * `len` - the number of bytes to copy.
/// * `ilet` - a pointer to an ilet
pub fn async_ldma_copy_ilet(dest: *mut c_void, src: *const c_void, len: u32, ilet: *mut simple_ilet) {
    unsafe {
        __async_ldma_copy_ilet(dest, src, len, ilet)
    }
}

/// Asynchronously copy memory using the local DMA engine.
/// The async_dma_transfer() function copies len bytes from memory area
/// src to memory area dest and schedules ilet when the copy is
/// finished.
///
/// # Arguments
///
/// * `src` - A pointer to the source buffer
/// * `dest` - A pointer to the destionation buffer
/// * `len` - the number of bytes to copy.
/// * `ilet` - a pointer to an ilet
///
/// # Deprecated
///
/// use async_ldma_copy() instead.
pub fn async_dma_transfer(src: *mut c_void, dest: *mut c_void, len: u32, ilet: *mut simple_ilet) {
    unsafe {
        __async_dma_transfer(src, dest, len, ilet)
    }
}

/// Set memory using the local DMA engine.
/// The sync_dma_memset() function set len bytes of the memory area dst to the value val.
///
/// # Arguments
///
/// * `dst` - A pointer to the destionation buffer
/// * `val` - The value to set
/// * `len` - the number of bytes to set
pub fn sync_dma_memset(dst: *mut c_void, val: u32, len: u32) {
    unsafe {
        __sync_dma_memset(dst, val, len)
    }
}

