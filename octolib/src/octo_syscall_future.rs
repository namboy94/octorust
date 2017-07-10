use constants::*;
use octo_types::*;
use octo_ilet::*;

#[repr(C)]
pub struct syscall_future {
    pub padding: [c_char; SYSCALL_FUTURE_SIZE],
}

extern {
    #[link_name="syscall_future_init"]
    fn __syscall_future_init(future: *mut syscall_future);

    #[link_name="syscall_future_init_ret_val"]
    fn __syscall_future_init_ret_val(future: *mut syscall_future, ret_val: *mut c_void);

    #[link_name="syscall_future_on_completion"]
    fn __syscall_future_on_completion(future: *mut syscall_future, ilet: *mut simple_ilet);

    #[link_name="syscall_future_wait"]
    fn __syscall_future_wait(future: *mut syscall_future);

}

/// Initialize a future.
///
/// # Arguments
///
/// * `future` - A pointer to the future to initialize.
///
pub fn syscall_future_init(future: *mut syscall_future) {
    unsafe {
        __syscall_future_init(future)
    }
}

/// Initialize a future.
///
/// # Arguments
///
/// * `future` - A pointer to the future to initialize.
/// * `ret_val` - A pointer to the memory area where the ret_val should be placed.
///
pub fn syscall_future_init_ret_val(future: *mut syscall_future, ret_val: *mut c_void) {
    unsafe {
        __syscall_future_init_ret_val(future, ret_val)
    }
}

/// Add a "on completion" iLet to the given future.
/// The given ilet will be execute once the future has completed. It's
/// save to add this ilet after the future has already been completed.
///
/// # Arguments
///
/// * `future` - A pointer to the future.
/// * `ilet` - The "on completion" ilet.
///
pub fn syscall_future_on_completion(future: *mut syscall_future, ilet: *mut simple_ilet) {
    unsafe {
        __syscall_future_on_completion(future, ilet)
    }
}

/// Await the completion of the future.
/// This function block until the future is completed.
///
/// # Arguments
///
/// * `future` - A pointer to the future.
///
pub fn syscall_future_wait(future: *mut syscall_future) {
    unsafe {
        __syscall_future_wait(future)
    }
}
