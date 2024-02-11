use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

/// A very basic [Mutex] implementation.
///
/// This implementation doesn't have any concept of detecting "poisoning" (when the current thread that
/// has locked them panics) which prevents other threads that are waiting on the mutex from accessing bad data.
pub struct Mutex<T> {
    // FIXME: When the MMU is enabled, make the Mutex actually lock.
    // is_locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new instance of [Mutex] which holds [data] of [T].
    pub const fn new(data: T) -> Mutex<T> {
        Self {
            // is_locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Attempts to take ownership of this [Mutex].
    ///
    /// If the mutex is currently owned by another thread, the thread will enter a spin-lock
    /// until the owner releases their lock (by dropping the [Guard]).
    pub fn lock(&self) -> Guard<T> {
        // while self.is_locked.swap(true, Ordering::AcqRel) {
        //     hint::spin_loop()
        // }

        Guard { mutex: self }
    }
}

/// This guard is given to the thread which locks a [Mutex] in order
/// to access its data.
///
/// This will automatically unlock the [mutex] when it is dropped, or goes
/// out of scope.
pub struct Guard<'a, T> {
    mutex: &'a Mutex<T>,
}

/// Allows the owner of the [Guard] to get a non-mutable reference to its value.
impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

/// Allows the owner of the [Guard] to get a mutable reference to its value.
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

/// Unlocks the [Mutex] when the the current [Guard] goes out of scope.
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        // self.mutex.is_locked.store(false, Ordering::Release);
    }
}

/// # Safety
/// - The [Mutex] doesn't allow the data of [T] to be accessed if it is locked by another thread.
///   It is thread safe.
unsafe impl<T: Send> Send for Mutex<T> {}

/// # Safety
/// - The [Mutex] doesn't allow the data of [T] to be accessed if it is locked by another thread.
///   It is thread safe.
unsafe impl<T: Send> Sync for Mutex<T> {}
