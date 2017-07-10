/// Provides clock() function to measure wall time.

pub type clock_t = u64;

extern {
    #![link_name="clock"]
    fn __clock() -> clock_t;
}

/// # Return Value
///
/// time since booting in microseconds
pub fn clock() -> clock_t {
    unsafe {
        __clock()
    }
}
