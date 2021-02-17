use core::hint::spin_loop;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub struct SpinLock {
    lock: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> SpinLock {
        SpinLock {
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while match self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) {
            Ok(x) => x,
            Err(x) => x,
        } {
            while self.lock.load(Ordering::Relaxed) {
                spin_loop();
            }
        }
    }

    pub fn unlock(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

pub struct ReentrantSpinLock {
    refcnt: Mutex<usize>,
    core: Option<esp32_hal::Core>,
}

impl ReentrantSpinLock {
    pub const fn new() -> ReentrantSpinLock {
        ReentrantSpinLock {
            refcnt: Mutex::new(0),
            core: None,
        }
    }

    /// Locks the spinlock
    ///
    /// Returns true if the spinlock was already held by this core, false if not.
    pub fn lock(&mut self) -> bool {
        loop {
            let mut refcnt = self.refcnt.lock();
            if *refcnt == 0 {
                self.core = Some(esp32_hal::get_core());
                *refcnt = 1;
                return false;
            } else {
                match self.core {
                    None => panic!("ReentrantSpinLock is locked but has no core"),
                    Some(core) if core == esp32_hal::get_core() => {
                        *refcnt += 1;
                        return true;
                    },
                    _ => {
                        drop(refcnt);
                        spin_loop();
                    },
                }
            }
        }
    }

    /// Unlock the spinlock
    ///
    /// Runs the provided closure before unlocking the spinlock if this call fully unlocks it.
    ///
    /// Panics if spinlock is not locked or if it is held by a different core.
    ///
    /// Returns true if spinlock is now unlocked, false if it is still held by this core.
    pub fn unlock<T>(&mut self, before_unlocking: impl FnOnce() -> T) -> bool {
        let mut refcnt = self.refcnt.lock();
        if *refcnt == 0 {
            panic!("ReentrantSpinLock is not locked");
        } else {
            match self.core {
                None => panic!("ReentrantSpinLock is locked but has no core"),
                Some(core) if core == esp32_hal::get_core() => {
                    *refcnt -= 1;
                    if *refcnt == 0 {
                        self.core = None;
                        before_unlocking();
                        return true;
                    } else {
                        return false;
                    }
                },
                _ => panic!("ReentrantSpinLock is locked by the other core"),
            }
        }
    }
}
