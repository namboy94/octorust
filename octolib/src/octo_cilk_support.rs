/// Support functions for the Cilk Plus runtime system.

extern {
    #[link_name="cilk_create_continuation"]
    fn __cilk_create_continuation();

    #[link_name="cilk_yield"]
    fn __cilk_yield();

    // TODO Figure this thing out
    // Original: void cilk_spawn_function(void (*func)(void *, void *), void *arg1, void *arg2);
    //#[link_name="cilk_spawn_function"]
    //fn __cilk_spawn_function(*: void (*func)(void, *): void, *arg1: void, *arg2: void);
}



/// Creates and enqueues a continuation.
/// This function must be called before cilk_yield(). Calling
/// simple_signal_wait() or any other blocking system call between
/// cilk_create_continuation() and cilk_yield() leads to undefined behaviour.
pub fn cilk_create_continuation() {
    unsafe {
        __cilk_create_continuation()
    }
}

/// Relinquishes the CPU, allowing another i-let to run.
/// If the current i-let is ever supposed to be resumed,
/// cilk_create_continuation() must be called before.
pub fn cilk_yield() {
    unsafe {
        __cilk_yield()
    }
}

/*
/// Spawns a function in a new i-let.
/// The current control-flow state is saved and a continuation is inserted into
/// the scheduler queue.
pub fn cilk_spawn_function(*: void (*func)(void, *): void, *arg1: void, *arg2: void) {
    unsafe {
        __cilk_spawn_function(*, *), *arg1, *arg2)
    }
}
*/
