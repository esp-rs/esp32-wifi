use alloc::boxed::Box;
use core::hint::spin_loop;
use crate::compatibility::interrupts::InterruptManager;

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

    pub unsafe fn use_raw<T>(address: *mut cty::c_void, f: impl FnOnce(&mut SpinLock) -> T) -> T {
        let mut spinlock = Box::from_raw(address as *mut SpinLock);
        let res = f(&mut spinlock);
        Box::leak(spinlock);
        res
    }

    pub fn try_acquire(&mut self) -> bool {
        let mut result = esp32_hal::get_core() as u32;
        let old_intr_level = InterruptManager::set_intlevel_excm();
        // SAFETY: low-level compare-and-set operation using known-correct pointers to active memory
        unsafe { _compare_and_set(&mut self.owner, SPINLOCK_FREE, &mut result) };
        let acquired = if result == SPINLOCK_FREE || result == esp32_hal::get_core() as u32 {
            self.count += 1;
            true
        } else {
            false
        };
        InterruptManager::restore_intlevel(old_intr_level);
        acquired
    }

    pub fn acquire(&mut self) {
        while !self.try_acquire() {
            spin_loop();
        }
    }

    pub fn release(&mut self) {
        let old_intr_level = InterruptManager::set_intlevel_excm();
        if self.owner != esp32_hal::get_core() as u32 {
            panic!("Attempt to release un-acquired spinlock");
        }
        self.count -= 1;
        if self.count == 0 {
            self.owner = SPINLOCK_FREE;
        }
        InterruptManager::restore_intlevel(old_intr_level);
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
