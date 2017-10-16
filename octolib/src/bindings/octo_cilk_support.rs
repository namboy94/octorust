/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_cilk_support.h file
///
/// Support functions for the Cilk Plus runtime system.

extern {

    /// Creates and enqueues a continuation.
    /// This function must be called before cilk_yield(). Calling
    /// simple_signal_wait() or any other blocking system call between
    /// cilk_create_continuation() and cilk_yield() leads to undefined behaviour.
    pub fn cilk_create_continuation();

    /// Relinquishes the CPU, allowing another i-let to run.
    /// If the current i-let is ever supposed to be resumed,
    /// cilk_create_continuation() must be called before.
    pub fn cilk_yield();

    /*
    /// Spawns a function in a new i-let.
    /// The current control-flow state is saved and a continuation is inserted into
    /// the scheduler queue.
    */
    // TODO void cilk_spawn_function(void (*func)(void *, void *), void *arg1, void *arg2);
}
