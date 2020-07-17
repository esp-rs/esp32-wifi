#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use cstr_core::CStr;

use esp32_hal::{
    clock_control::{sleep, ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    dprintln,
    prelude::*,
    serial::{config::Config, Pins, Serial},
    target,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    let (mut dport, dport_clock_control) = dp.DPORT.split();

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
            baudrate: 115200.Hz(),
            ..Config::default()
        },
        clkcntrl_config,
        &mut dport,
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
        writeln!(
            serial,
            "Wifi set_log_mod result: {:8x}",
            esp32_wifi::binary::wifi::esp_wifi_internal_set_log_mod(
                esp32_wifi::binary::wifi::wifi_log_module_t::WIFI_LOG_MODULE_ALL,
                0,
                true
            )
        )
        .unwrap();

        esp32_wifi::wifi::WiFi::new(clkcntrl_config);

        // Needed to force inclusion of compatibility module: have not found other way yet.
        writeln!(
            serial,
            "Wifi set_log_mod result: {:?}",
            esp32_wifi::compatibility::implicit::WIFI_EVENT
        )
        .unwrap();
    }

    loop {
        sleep(1.s());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dprintln!("\n\n*** {:?}", info);
    loop {}
}
