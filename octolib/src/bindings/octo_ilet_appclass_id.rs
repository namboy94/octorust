/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_ilet_appclass.h file

use octo_types::*;
use octo_structs::*;

extern {

    ///  Initialise simple ilet(one parameter form) including id and applclass
    ///
    /// # Arguments
    ///
    /// * `ilet` - Pointer to ilet instance
    /// * `code` - Pointer to function
    /// * `param` - parameter for function
    /// * `id` - id to be written into ilet
    /// * `appl_class` - application clas to be written into ilet
    pub fn simple_ilet_init_appclass(ilet: *mut simple_ilet, code: ilet_func, param: *mut c_void, id: u32, appl_class: u32);

    ///  Initialise dual ilet(two parameter form) including id and applclass
    ///
    /// # Arguments
    ///
    /// * `ilet` - Pointer to ilet instance
    /// * `code` - Pointer to function
    /// * `param1` - first parameter for function
    /// * `param2` - second parameter for function
    /// * `id` - id to be written into ilet
    /// * `appl_class` - application clas to be written into ilet
    pub fn dual_ilet_init_appclass(ilet: *mut simple_ilet, code: dual_ilet_func, param1: *mut c_void, param2: *mut c_void, id: u32, appl_class: u32);

    ///  Program monitor weights of an application class
    ///
    /// # Arguments
    ///
    /// * `class_index` - Index of application class
    /// * `class_weights` - Every nibble represents a weight for the corresponding
    ///                     monitor value
    ///
    /// # Return Value
    ///
    ///  0 on success, -1 on failure
    pub fn program_application_class(class_index: u32, class_weights: u32) -> i32;

}
