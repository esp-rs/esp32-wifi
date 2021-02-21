use core::hint::spin_loop;
use spin::Mutex;

/// 'owner' value that indicates the spinlock is unheld
const SPINLOCK_FREE: u32 = 0xB33FFFFF;

/// This structure has the memory layout that is expected by the
/// binary ESP-IDF WiFi driver.
#[repr(C)]
pub struct SpinLock {
    owner: u32,
    count: u32,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            owner: SPINLOCK_FREE,
            count: 0,
        }
    }

    pub fn try_acquire(&mut self) -> bool {
        let mut result = esp32_hal::get_core() as u32;
        let int_mask = xtensa_lx6::interrupt::disable();
        // SAFETY: low-level compare-and-set operation using known-correct pointers to active memory
        unsafe { _compare_and_set(&mut self.owner, SPINLOCK_FREE, &mut result) };
        let acquired = if result == SPINLOCK_FREE || result == esp32_hal::get_core() as u32 {
            self.count += 1;
            true
        } else {
            false
        };
        // SAFETY: restoring previously-set interrupt mask
        unsafe { xtensa_lx6::interrupt::set_mask(int_mask) };
        acquired
    }

    pub fn acquire(&mut self) {
        while !self.try_acquire() {
            spin_loop();
        }
    }

    pub fn release(&mut self) {
        let int_mask = xtensa_lx6::interrupt::disable();
        if self.owner != esp32_hal::get_core() as u32 {
            panic!("Attempt to release un-acquired spinlock");
        }
        self.count -= 1;
        if self.count == 0 {
            self.owner = SPINLOCK_FREE;
        }
        // SAFETY: restoring previously-set interrupt mask
        unsafe { xtensa_lx6::interrupt::set_mask(int_mask) };
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

// SAFETY: Since we have to use 'raw' values for the spinlock members, we can't use rust's
// SAFETY: AtomicUInt, so we have to do compare-and-set 'manually'.
unsafe fn _compare_and_set(addr: *mut u32, compare: u32, set: *mut u32) {
    llvm_asm!(r#"
        wsr $2, SCOMPARE1
        s32c1i $0, $1, 0"#
        : "=r"(*set)
        : "r"(addr), "r"(compare), "0"(*set)
        :: "volatile");
}
