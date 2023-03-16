use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

#[derive(Debug)]
struct SpinLockedInt {
    locked: AtomicBool,
    value: UnsafeCell<f32>,
}

// Unsafe means: "Trust me, I will make sure concurrent access is safe"
unsafe impl Sync for SpinLockedInt {}

impl SpinLockedInt {
    pub fn new(value: f32) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_> {
        // Swap returns the previous value. If the previous value
        // was locked, we keep spinning, if it was unlocked, we set locked
        // in and break the while loop
        while self.locked.swap(LOCKED, Ordering::Relaxed) {
            // Acquire
            thread::yield_now(); // allow other threads to get in the way
        }
        SpinLockGuard { spin_lock: self }
    }

    // Buggy method
    pub fn lock_badly(&self) -> SpinLockGuard<'_> {
        // Spin until lock is available
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {
            thread::yield_now(); // allow other threads to get in the way
        }
        thread::yield_now();
        self.locked.store(LOCKED, Ordering::Relaxed);

        SpinLockGuard { spin_lock: self }
    }
}

struct SpinLockGuard<'a> {
    spin_lock: &'a SpinLockedInt,
}

impl SpinLockGuard<'_> {
    fn get(&self) -> &f32 {
        unsafe { &*self.spin_lock.value.get() }
    }

    fn get_mut(&mut self) -> &mut f32 {
        unsafe { &mut *self.spin_lock.value.get() }
    }
}

impl Deref for SpinLockGuard<'_> {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.value.get() }
    }
}

impl DerefMut for SpinLockGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.value.get() }
    }
}

impl Drop for SpinLockGuard<'_> {
    fn drop(&mut self) {
        self.spin_lock.locked.store(UNLOCKED, Ordering::Relaxed); // Should be Release
    }
}

// Unsafe means: "Trust me, I will make sure concurrent access is safe"
unsafe impl Sync for SpinLockGuard<'_> {}

fn main() {
    let value = SpinLockedInt::new(0.0);
    thread::scope(|s| {
        let thread_handles = (0..100).map(|_| {
            s.spawn(|| {
                for _ in 0..100 {
                    let mut lock = value.lock_badly();
                    *(lock.get_mut()) += 1.0;
                }
            })
        });

        thread_handles.into_iter().for_each(|t| {
            t.join().unwrap();
        });
    });
    println!("{}", *value.lock());
}
