/// Macro for logging WIFI info.
///
use xtensa_lx6::mutex::CriticalSectionSpinLockMutex;

pub static LOCK: CriticalSectionSpinLockMutex<()> = CriticalSectionSpinLockMutex::new(());

#[macro_export]
macro_rules! fwprintln {
    ($($arg:tt)*) => {
        {
            use esp32_hal::{dprint, dprintln};
            use xtensa_lx6::mutex::mutex_trait::Mutex;
            (&crate::log::LOCK).lock(|_| {
                dprint!("WIFI core {:?}: ",esp32_hal::get_core());
                dprintln!($($arg)*);
            });
        }
    };
}

#[macro_export]
macro_rules! wprintln {
    ($($arg:tt)*) => {
        // crate::fwprintln!($($arg)*);
    };
}
