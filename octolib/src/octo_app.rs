/// Functions for application management

extern {
    #[link_name="octo_app_protected"]
    fn __octo_app_protected() -> i32;

    #[link_name="octo_app_unprotect"]
    fn __octo_app_unprotect();

    #[link_name="octo_app_protect"]
    fn __octo_app_protect();

}

/// Retrieve protection status of the current application.
/// # Return Value
///
/// 1 if protection is active, 0 otherwise.
///
pub fn octo_app_protected() -> i32 {
    unsafe {
        __octo_app_protected()
    }
}

/// Drop memory protection for the calling application on the local tile.
///        The application gains access to all memory ranges and may gain kernel
///        privileges easily. This function is for testing purposes, do not use
///        it.
pub fn octo_app_unprotect() {
    unsafe {
        __octo_app_unprotect()
    }
}

/// Restrict the calling application on the local tile to its valid
///        memory regions. This is already active when the application starts.
pub fn octo_app_protect() {
    unsafe {
        __octo_app_protect()
    }
}
