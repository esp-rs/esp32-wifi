/// Macro for logging WIFI info.
///
use crate::compatibility::spinlock::SpinLock;

pub static mut LOCK: SpinLock = SpinLock::new();

#[macro_export]
macro_rules! fwprintln {
    ($($arg:tt)*) => {
        {
            use esp32_hal::{dprint, dprintln};
            use crate::compatibility::interrupts::InterruptManager;
            InterruptManager::run(|mgr| {
                #[allow(unused_unsafe)]
                unsafe { mgr.enter_critical(&mut crate::log::LOCK) };
                dprint!("WIFI core {:?}: ",esp32_hal::get_core());
                dprintln!($($arg)*);
                #[allow(unused_unsafe)]
                unsafe { mgr.exit_critical(&mut crate::log::LOCK) };
            });
        }
    };
}

#[macro_export]
macro_rules! wprintln {
    ($($arg:tt)*) => {
        crate::fwprintln!($($arg)*);
    };
}
