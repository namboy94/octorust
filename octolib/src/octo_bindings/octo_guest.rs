extern {
    #[link_name="shutdown"] fn __shutdown(exitcode: i32);
}

pub fn shutdown(exitcode: i32) { unsafe { __shutdown(exitcode) } }