/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_app.h file
///
/// Functions for application management

extern {
    /// Retrieve protection status of the current application.
    ///
    /// # Return Value
    ///
    /// 1 if protection is active, 0 otherwise.
    pub fn octo_app_protected() -> i32;

    /// Drop memory protection for the calling application on the local tile.
    /// The application gains access to all memory ranges and may gain kernel
    /// privileges easily. This function is for testing purposes, do not use it.
    pub fn octo_app_unprotect();

    /// Restrict the calling application on the local tile to its valid
    /// memory regions. This is already active when the application starts.
    pub fn octo_app_protect();

}
