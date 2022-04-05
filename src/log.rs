/// Macro for logging WIFI info.
///
use crate::compatibility::spinlock::SpinLock;

#[derive(PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum LogLevel {
    NONE = 0,
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5,
}

pub(crate) const LOG_LEVEL: LogLevel = LogLevel::DEBUG;
pub(crate) static mut LOCK: SpinLock = SpinLock::new();

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
macro_rules! weprintln {
    ($($arg:tt)*) => {
        if crate::log::LOG_LEVEL >= crate::log::LogLevel::ERROR {
            crate::fwprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! wwprintln {
    ($($arg:tt)*) => {
        if crate::log::LOG_LEVEL >= crate::log::LogLevel::WARN {
            crate::fwprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! wiprintln {
    ($($arg:tt)*) => {
        if crate::log::LOG_LEVEL >= crate::log::LogLevel::INFO {
            crate::fwprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! wdprintln {
    ($($arg:tt)*) => {
        if crate::log::LOG_LEVEL >= crate::log::LogLevel::DEBUG {
            crate::fwprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! wtprintln {
    ($($arg:tt)*) => {
        if crate::log::LOG_LEVEL >= crate::log::LogLevel::TRACE {
            crate::fwprintln!($($arg)*);
        }
    };
}
