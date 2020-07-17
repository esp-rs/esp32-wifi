use cty::{c_char, c_int, c_uint, c_void};
use esp32_hal::{dprint, dprintln};

#[repr(C)]
#[derive(Debug)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Functions from esp-idf
#[no_mangle]
pub static WIFI_EVENT: StaticCString = StaticCString(b"WIFI_EVENT\0" as *const u8);

#[no_mangle]
pub unsafe extern "C" fn phy_enter_critical() -> c_uint {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn phy_exit_critical(_level: c_uint) {
    unimplemented!();
}

#[no_mangle]
unsafe extern "C" fn phy_printf(fmt: *const c_char, ...) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe extern "C" fn net80211_printf(fmt: *const c_char, ...) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe extern "C" fn hexstr2bin(hex: *const c_char, buf: *const u8, len: usize) -> c_int {
    unimplemented!();
}

// Functions available in ROM

#[no_mangle]
pub unsafe extern "C" fn roundup2(x: c_int, size: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn __popcountsi2(x: c_int) -> c_uint {
    dprintln!("COMPATIBILITY: __popcountsi2({})", x);
    x.count_ones()
}

#[no_mangle]
pub unsafe extern "C" fn gpio_output_set(
    set_mask: c_uint,
    clear_mask: c_uint,
    enable_mask: c_uint,
    disable_mask: c_uint,
) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gpio_output_set_high(
    set_mask: c_uint,
    clear_mask: c_uint,
    enable_mask: c_uint,
    disable_mask: c_uint,
) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn intr_matrix_set(cpu_no: c_int, model_num: c_uint, intr_num: c_uint) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn ets_delay_us(us: c_uint) {
    unimplemented!();
}

// Functions from libc

#[no_mangle]
pub unsafe extern "C" fn strnlen(cs: *const c_char, maxlen: usize) -> usize {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn strlen(cs: *const c_char) -> usize {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const c_void) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn abort() -> ! {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(cs: *const c_char, ct: *const c_char, n: usize) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn puts(a: *const c_char) -> c_int {
    dprintln!("WIFI: {}", cstr_core::CStr::from_ptr(a).to_str().unwrap());
    true as c_int
}
