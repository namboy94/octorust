extern {
    #![link_name="debug_print_blocked_ilets"]
    fn __debug_print_blocked_ilets();
}

pub fn debug_print_blocked_ilets() {
    unsafe {
        __debug_print_blocked_ilets()
    }
}