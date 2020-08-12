#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::{fmt::Write, panic::PanicInfo};
use cstr_core::CStr;

use esp32_hal::{
    clock_control::{sleep, ClockControl, ClockControlConfig, XTAL_FREQUENCY_AUTO},
    dport::Split,
    dprintln,
    prelude::*,
    serial::{config::Config, Pins, Serial},
    target,
    timer::Timer,
};

use esp32_hal::alloc::{Allocator, DEFAULT_ALLOCATOR};

#[global_allocator]
pub static GLOBAL_ALLOCATOR: Allocator = DEFAULT_ALLOCATOR;

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!(
        "Error allocating  {} bytes of memory with alignment {}",
        layout.size(),
        layout.align()
    );
}

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    let (_, dport_clock_control) = dp.DPORT.split();

    let clkcntrl = ClockControl::new(
        dp.RTCCNTL,
        dp.APB_CTRL,
        dport_clock_control,
        XTAL_FREQUENCY_AUTO,
    )
    .unwrap();

    let (clkcntrl_config, mut watchdog) = clkcntrl.freeze().unwrap();
    watchdog.disable();

    let (_, _, _, mut watchdog0) = Timer::new(dp.TIMG0, clkcntrl_config);
    let (_, _, _, mut watchdog1) = Timer::new(dp.TIMG1, clkcntrl_config);
    watchdog0.disable();
    watchdog1.disable();

    let pins = dp.GPIO.split();

    let mut serial: Serial<_, _, _> = Serial::new(
        dp.UART0,
        Pins {
            tx: pins.gpio1,
            rx: pins.gpio3,
            cts: None,
            rts: None,
        },
        Config {
            // default configuration is 19200 baud, 8 data bits, 1 stop bit & no parity (8N1)
            baudrate: 921600.Hz(),
            ..Config::default()
        },
        clkcntrl_config,
    )
    .unwrap();

    writeln!(serial, "\n\nESP32 Started\n\n").unwrap();

    unsafe {
        writeln!(
            serial,
            "Coexist library version: {}",
            CStr::from_ptr(esp32_wifi::binary::coexist::coex_version_get())
                .to_str()
                .unwrap()
        )
        .unwrap();
        writeln!(
            serial,
            "Phy RF calibration data version: {}",
            esp32_wifi::binary::phy::phy_get_rf_cal_version()
        )
        .unwrap();
        writeln!(
            serial,
            "Wifi set_log_level result: {:8x}",
            esp32_wifi::binary::wifi::esp_wifi_internal_set_log_level(
                esp32_wifi::binary::wifi::wifi_log_level_t::WIFI_LOG_VERBOSE
            )
        )
        .unwrap();

        writeln!(serial, "WiFi::new:").unwrap();

        let wifi = match esp32_wifi::wifi::WiFi::new(clkcntrl_config) {
            Ok(wifi) => wifi,
            Err(err) => panic!("Error starting wifi: {:?}", err),
        };

        writeln!(serial, "set_mode:").unwrap();

        wifi.set_mode(esp32_wifi::wifi::Mode::WIFI_MODE_STA)
            .unwrap();

        writeln!(serial, "set_station_config:").unwrap();

        wifi.set_station_config(&mut esp32_wifi::binary::wifi::wifi_sta_config_t {
            ..Default::default()
        })
        .unwrap();

        writeln!(serial, "start:").unwrap();

        wifi.start().unwrap();

        writeln!(serial, "\n\nFinished wifi calls").unwrap();
    }

    writeln!(serial, "\n\nEntering loop...").unwrap();

    loop {
        sleep(1.s());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // park the other core
    unsafe { ClockControlConfig {}.park_core(esp32_hal::get_other_core()) };

    // print panic message
    dprintln!("\n\n*** Core: {:?} {:?}", esp32_hal::get_core(), info);

    // park this core
    unsafe { ClockControlConfig {}.park_core(esp32_hal::get_core()) };

    dprintln!("\n\n Should not reached because core is parked!!!");

    // this statement will not be reached, but is needed to make this a diverging function
    loop {}
}
