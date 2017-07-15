void simple_spinlock_init(simple_spinlock* instance);

void simple_spinlock_lock(simple_spinlock* instance);

void simple_spinlock_unlock(simple_spinlock* instance);

int simple_spinlock_trylock(simple_spinlock* instance);

int simple_spinlock_isLocked(simple_spinlock* instance);

void recursive_spinlock_init(recursive_spinlock* instance);

void recursive_spinlock_lock(recursive_spinlock* instance);

int recursive_spinlock_trylock(recursive_spinlock* instance);

int recursive_spinlock_isLocked(recursive_spinlock* instance);

void recursive_spinlock_unlock(recursive_spinlock* instance);
