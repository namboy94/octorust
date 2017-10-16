/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_ilet.h file
///
/// Functions regarding iLets
/// An iLet is the basic abstraction of an user control flow in OctoPOS.

use octo_types::*;
use octo_structs::*;

extern {

    /// Initialises a given iLet data structure.
    ///
    /// # Arguments
    ///
    /// * `ilet` - Pointer to simple_ilet instance
    /// * `code` - Pointer to a function to be executed by the iLet
    /// * `param` - Parameter passed to the function.
    pub fn simple_ilet_init(ilet: *mut simple_ilet, code: ilet_func, param: *mut c_void);

    pub fn dual_ilet_init(ilet: *mut simple_ilet, code: dual_ilet_func, param1: *mut c_void, param2: *mut c_void);

    /// Returns a pointer to the beginning of the i-let-local data area.
    /// Data stored in this area is guaranteed to be valid throughout an i-let's
    /// lifetime.
    /// OctoPOS guarantees that the area is at least 256 bytes large. The concrete
    /// size can be dermined by calling get_ilocal_data_size(). Accesses beyond the
    /// end of the area cause undefined behaviour.
    ///
    /// # Return Value
    ///
    /// Pointer to i-let-local data. It is the caller's responsibility to
    /// cast this pointer to the desired type.
    pub fn get_ilocal_data() -> *mut c_void;

    /// Returns the size of the current i-let's local data area.
    pub fn get_ilocal_data_size() -> usize;

    /// Sets the scheduling domain of the current i-let.
    pub fn set_cur_ilet_sched_dom(sched_dom: u8);

    /// Returns the scheduling domain of the current i-let.
    pub fn get_cur_ilet_sched_dom() -> u8;

    /// Sets the scheduling domain of the given i-let.
    pub fn set_ilet_sched_dom(ilet: *mut simple_ilet, sched_dom: u8);

    /// Returns the scheduling domain of the given i-let.
    pub fn get_ilet_sched_dom(ilet: *mut simple_ilet) -> u8;

}

