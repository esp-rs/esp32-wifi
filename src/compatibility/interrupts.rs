use crate::compatibility::spinlock::SpinLock;

pub const EXCM_LEVEL: u32 = 3;

static mut INTERRUPT_MANAGER: InterruptManager = InterruptManager::new();

struct CriticalState {
    nest_level: u32,
    saved_interrupt_state: u32,
}

impl CriticalState {
    const fn new() -> Self {
        Self {
            nest_level: 0,
            saved_interrupt_state: 0,
        }
    }
}

pub struct InterruptManager {
    critical_state: [CriticalState; 2],
}

impl InterruptManager {
    const fn new() -> Self {
        InterruptManager {
            critical_state: [CriticalState::new(), CriticalState::new()],
        }
    }

    pub fn run<T>(f: impl FnOnce(&mut InterruptManager) -> T) -> T {
        // SAFETY: accessing static mut var only happens here
        unsafe {
            f(&mut INTERRUPT_MANAGER)
        }
    }

    pub fn set_intlevel_excm() -> u32 {
        // SAFETY: we take no outside inputs here
        unsafe {
            #[allow(unused_assignments)]
            let mut old_intr_level: u32 = 0;
            llvm_asm!(r#"
                rsil $0, 3"#
            : "=a"(old_intr_level) :: "memory" : "volatile");
            old_intr_level
        }
    }

    pub fn restore_intlevel(level: u32) {
        // SAFETY: caller is expected to pass a sane value for 'level'
        unsafe {
            llvm_asm!(r#"
                wsr.ps $0
                rsync
            "#
            :: "a"(level) : "memory" : "volatile");
        };
    }

    pub fn enter_critical_nested() -> u32 {
        InterruptManager::set_intlevel_excm()
    }

    pub fn exit_critical_nested(level: u32) {
        InterruptManager::restore_intlevel(level);
    }

    pub fn enter_critical(&mut self, spinlock: &mut SpinLock /* FIXME: should be a mutex? */) {
        let old_intr_level = InterruptManager::enter_critical_nested();

        spinlock.acquire();
        let core = esp32_hal::get_core() as usize;

        let new_nesting = self.critical_state[core].nest_level + 1;
        self.critical_state[core].nest_level = new_nesting;

        if new_nesting == 1 {
            self.critical_state[core].saved_interrupt_state = old_intr_level;
        }
    }

    pub fn exit_critical(&mut self, spinlock: &mut SpinLock /* FIXME: mutex? */) {
        spinlock.release();
        let core = esp32_hal::get_core() as usize;

        let mut nesting = self.critical_state[core].nest_level;
        if nesting > 0 {
            nesting -= 1;
            self.critical_state[core].nest_level = nesting;
            if nesting == 0 {
                InterruptManager::exit_critical_nested(self.critical_state[core].saved_interrupt_state);
            }
        }
    }
}
