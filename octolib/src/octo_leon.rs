/// Functions only meaningful in LEON version
/// LEON specific functions for platforms with APBUART

use octo_types::*;

extern {
    #[link_name="leon_set_bus_frequency"]
    fn __leon_set_bus_frequency(f: u32);

    #[link_name="leon_set_baud_rate"]
    fn __leon_set_baud_rate(rate: u32);

    #[link_name="leon_set_uart_debug_mode"]
    fn __leon_set_uart_debug_mode(m: i32);

    #[link_name="leon_get_noc_ts"]
    fn __leon_get_noc_ts() -> u64;

    #[link_name="chipscope_trigger"]
    fn __chipscope_trigger(value: u32);

}

/// set the bus frequency of your system (in Hz)
pub fn leon_set_bus_frequency(f: u32) {
    unsafe {
        __leon_set_bus_frequency(f)
    }
}

pub fn leon_set_baud_rate(rate: u32) {
    unsafe {
        __leon_set_baud_rate(rate)
    }
}

/// set FIFO Debug mode of APBUART:
/// 0: disable debug mode; output is set through normal serial interface
/// 1: enable debug mode; output can be viewed with grmon -u
pub fn leon_set_uart_debug_mode(m: i32) {
    unsafe {
        __leon_set_uart_debug_mode(m)
    }
}

/// Get NoC timestamp counter
///
/// # Return Value
///
/// value of the NoC's timestamp counter
pub fn leon_get_noc_ts() -> u64 {
    unsafe {
        __leon_get_noc_ts()
    }
}

/// Issues a trigger condition for ChipScope debugging.
pub fn chipscope_trigger(value: u32) {
    unsafe {
        __chipscope_trigger(value)
    }
}
