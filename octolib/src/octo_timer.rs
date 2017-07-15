use octo_types::*;

extern {
    #[link_name="timer_start"]
    fn __timer_start() -> uint64_t;

    #[link_name="timer_stop"]
    fn __timer_stop() -> uint64_t;

    #[link_name="ticks_to_nanoseconds"]
    fn __ticks_to_nanoseconds(ticks: uint64_t) -> uint64_t;

}

pub fn timer_start() -> uint64_t {
    unsafe {
        __timer_start()
    }
}

pub fn timer_stop() -> uint64_t {
    unsafe {
        __timer_stop()
    }
}

pub fn ticks_to_nanoseconds(ticks: uint64_t) -> uint64_t {
    unsafe {
        __ticks_to_nanoseconds(ticks)
    }
}

