use crate::binary::wifi::__va_list_tag;

pub fn init_osi_funcs() -> crate::binary::wifi::wifi_osi_funcs_t {
    crate::binary::wifi::wifi_osi_funcs_t {
        _version: 0x00000004,
        _set_isr: Some(_set_isr),
        _ints_on: Some(_ints_on),
        _ints_off: Some(_ints_off),
        _spin_lock_create: Some(_spin_lock_create),
        _spin_lock_delete: Some(_spin_lock_delete),
        _wifi_int_disable: Some(_wifi_int_disable),
        _wifi_int_restore: Some(_wifi_int_restore),
        _task_yield_from_isr: Some(_task_yield_from_isr),
        _semphr_create: Some(_semphr_create),
        _semphr_delete: Some(_semphr_delete),
        _semphr_take: Some(_semphr_take),
        _semphr_give: Some(_semphr_give),
        _wifi_thread_semphr_get: Some(_wifi_thread_semphr_get),
        _mutex_create: Some(_mutex_create),
        _recursive_mutex_create: Some(_recursive_mutex_create),
        _mutex_delete: Some(_mutex_delete),
        _mutex_lock: Some(_mutex_lock),
        _mutex_unlock: Some(_mutex_unlock),
        _queue_create: Some(_queue_create),
        _queue_delete: Some(_queue_delete),
        _queue_send: Some(_queue_send),
        _queue_send_from_isr: Some(_queue_send_from_isr),
        _queue_send_to_back: Some(_queue_send_to_back),
        _queue_send_to_front: Some(_queue_send_to_front),
        _queue_recv: Some(_queue_recv),
        _queue_msg_waiting: Some(_queue_msg_waiting),
        _event_group_create: Some(_event_group_create),
        _event_group_delete: Some(_event_group_delete),
        _event_group_set_bits: Some(_event_group_set_bits),
        _event_group_clear_bits: Some(_event_group_clear_bits),
        _event_group_wait_bits: Some(_event_group_wait_bits),
        _task_create_pinned_to_core: Some(_task_create_pinned_to_core),
        _task_create: Some(_task_create),
        _task_delete: Some(_task_delete),
        _task_delay: Some(_task_delay),
        _task_ms_to_tick: Some(_task_ms_to_tick),
        _task_get_current_task: Some(_task_get_current_task),
        _task_get_max_priority: Some(_task_get_max_priority),
        _malloc: Some(_malloc),
        _free: Some(_free),
        _event_post: Some(_event_post),
        _get_free_heap_size: Some(_get_free_heap_size),
        _rand: Some(_rand),
        _dport_access_stall_other_cpu_start_wrap: Some(_dport_access_stall_other_cpu_start_wrap),
        _dport_access_stall_other_cpu_end_wrap: Some(_dport_access_stall_other_cpu_end_wrap),
        _phy_rf_deinit: Some(_phy_rf_deinit),
        _phy_load_cal_and_init: Some(_phy_load_cal_and_init),
        _phy_common_clock_enable: Some(_phy_common_clock_enable),
        _phy_common_clock_disable: Some(_phy_common_clock_disable),
        _read_mac: Some(_read_mac),
        _timer_arm: Some(_timer_arm),
        _timer_disarm: Some(_timer_disarm),
        _timer_done: Some(_timer_done),
        _timer_setfn: Some(_timer_setfn),
        _timer_arm_us: Some(_timer_arm_us),
        _periph_module_enable: Some(_periph_module_enable),
        _periph_module_disable: Some(_periph_module_disable),
        _esp_timer_get_time: Some(_esp_timer_get_time),
        _nvs_set_i8: Some(_nvs_set_i8),
        _nvs_get_i8: Some(_nvs_get_i8),
        _nvs_set_u8: Some(_nvs_set_u8),
        _nvs_get_u8: Some(_nvs_get_u8),
        _nvs_set_u16: Some(_nvs_set_u16),
        _nvs_get_u16: Some(_nvs_get_u16),
        _nvs_open: Some(_nvs_open),
        _nvs_close: Some(_nvs_close),
        _nvs_commit: Some(_nvs_commit),
        _nvs_set_blob: Some(_nvs_set_blob),
        _nvs_get_blob: Some(_nvs_get_blob),
        _nvs_erase_key: Some(_nvs_erase_key),
        _get_random: Some(_get_random),
        _get_time: Some(_get_time),
        _random: Some(_random),
        _log_write: Some(_log_write),
        _log_writev: Some(_log_writev),
        _log_timestamp: Some(_log_timestamp),
        _malloc_internal: Some(_malloc_internal),
        _realloc_internal: Some(_realloc_internal),
        _calloc_internal: Some(_calloc_internal),
        _zalloc_internal: Some(_zalloc_internal),
        _wifi_malloc: Some(_wifi_malloc),
        _wifi_realloc: Some(_wifi_realloc),
        _wifi_calloc: Some(_wifi_calloc),
        _wifi_zalloc: Some(_wifi_zalloc),
        _wifi_create_queue: Some(_wifi_create_queue),
        _wifi_delete_queue: Some(_wifi_delete_queue),
        _modem_sleep_enter: Some(_modem_sleep_enter),
        _modem_sleep_exit: Some(_modem_sleep_exit),
        _modem_sleep_register: Some(_modem_sleep_register),
        _modem_sleep_deregister: Some(_modem_sleep_deregister),
        _coex_status_get: Some(_coex_status_get),
        _coex_condition_set: Some(_coex_condition_set),
        _coex_wifi_request: Some(_coex_wifi_request),
        _coex_wifi_release: Some(_coex_wifi_release),
        _magic: 0xDEADBEAFu32 as i32,
    }
}

pub unsafe extern "C" fn _set_isr(n: i32, f: *mut cty::c_void, arg: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _ints_on(mask: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _ints_off(mask: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _spin_lock_create() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _spin_lock_delete(lock: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_int_disable(wifi_int_mux: *mut cty::c_void) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_int_restore(wifi_int_mux: *mut cty::c_void, tmp: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _task_yield_from_isr() {
    unimplemented!()
}
pub unsafe extern "C" fn _semphr_create(max: u32, init: u32) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _semphr_delete(semphr: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _semphr_take(semphr: *mut cty::c_void, block_time_tick: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _semphr_give(semphr: *mut cty::c_void) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_thread_semphr_get() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _mutex_create() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _recursive_mutex_create() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _mutex_delete(mutex: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _mutex_lock(mutex: *mut cty::c_void) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _mutex_unlock(mutex: *mut cty::c_void) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_create(queue_len: u32, item_size: u32) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_delete(queue: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send(
    queue: *mut cty::c_void,
    item: *mut cty::c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send_from_isr(
    queue: *mut cty::c_void,
    item: *mut cty::c_void,
    hptw: *mut cty::c_void,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send_to_back(
    queue: *mut cty::c_void,
    item: *mut cty::c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send_to_front(
    queue: *mut cty::c_void,
    item: *mut cty::c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_recv(
    queue: *mut cty::c_void,
    item: *mut cty::c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_msg_waiting(queue: *mut cty::c_void) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_create() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_delete(event: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_set_bits(event: *mut cty::c_void, bits: u32) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_clear_bits(event: *mut cty::c_void, bits: u32) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_wait_bits(
    event: *mut cty::c_void,
    bits_to_wait_for: u32,
    clear_on_exit: i32,
    wait_for_all_bits: i32,
    block_time_tick: u32,
) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _task_create_pinned_to_core(
    task_func: *mut cty::c_void,
    name: *const cty::c_char,
    stack_depth: u32,
    param: *mut cty::c_void,
    prio: u32,
    task_handle: *mut cty::c_void,
    core_id: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _task_create(
    task_func: *mut cty::c_void,
    name: *const cty::c_char,
    stack_depth: u32,
    param: *mut cty::c_void,
    prio: u32,
    task_handle: *mut cty::c_void,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _task_delete(task_handle: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _task_delay(tick: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _task_ms_to_tick(ms: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _task_get_current_task() -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _task_get_max_priority() -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _malloc(size: u32) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _free(p: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _event_post(
    event_base: *const cty::c_char,
    event_id: i32,
    event_data: *mut cty::c_void,
    event_data_size: usize,
    ticks_to_wait: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _get_free_heap_size() -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _rand() -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _dport_access_stall_other_cpu_start_wrap() {
    unimplemented!()
}
pub unsafe extern "C" fn _dport_access_stall_other_cpu_end_wrap() {
    unimplemented!()
}
pub unsafe extern "C" fn _phy_rf_deinit(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _phy_load_cal_and_init(module: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _phy_common_clock_enable() {
    unimplemented!()
}
pub unsafe extern "C" fn _phy_common_clock_disable() {
    unimplemented!()
}
pub unsafe extern "C" fn _read_mac(mac: *mut u8, type_: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _timer_arm(timer: *mut cty::c_void, tmout: u32, repeat: bool) {
    unimplemented!()
}
pub unsafe extern "C" fn _timer_disarm(timer: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _timer_done(ptimer: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _timer_setfn(
    ptimer: *mut cty::c_void,
    pfunction: *mut cty::c_void,
    parg: *mut cty::c_void,
) {
    unimplemented!()
}
pub unsafe extern "C" fn _timer_arm_us(ptimer: *mut cty::c_void, us: u32, repeat: bool) {
    unimplemented!()
}
pub unsafe extern "C" fn _periph_module_enable(periph: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _periph_module_disable(periph: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _esp_timer_get_time() -> i64 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_set_i8(handle: u32, key: *const cty::c_char, value: i8) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_get_i8(
    handle: u32,
    key: *const cty::c_char,
    out_value: *mut i8,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_set_u8(handle: u32, key: *const cty::c_char, value: u8) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_get_u8(
    handle: u32,
    key: *const cty::c_char,
    out_value: *mut u8,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_set_u16(handle: u32, key: *const cty::c_char, value: u16) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_get_u16(
    handle: u32,
    key: *const cty::c_char,
    out_value: *mut u16,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_open(
    name: *const cty::c_char,
    open_mode: u32,
    out_handle: *mut u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_close(handle: u32) {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_commit(handle: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_set_blob(
    handle: u32,
    key: *const cty::c_char,
    value: *const cty::c_void,
    length: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_get_blob(
    handle: u32,
    key: *const cty::c_char,
    out_value: *mut cty::c_void,
    length: *mut usize,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_erase_key(handle: u32, key: *const cty::c_char) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _get_random(buf: *mut u8, len: usize) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _get_time(t: *mut cty::c_void) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _random() -> cty::c_ulong {
    unimplemented!()
}
pub unsafe extern "C" fn _log_write(
    level: u32,
    tag: *const cty::c_char,
    format: *const cty::c_char,
    ...
) {
    unimplemented!()
}
pub unsafe extern "C" fn _log_writev(
    level: u32,
    tag: *const cty::c_char,
    format: *const cty::c_char,
    args: *mut __va_list_tag,
) {
    unimplemented!()
}
pub unsafe extern "C" fn _log_timestamp() -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _malloc_internal(size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _realloc_internal(ptr: *mut cty::c_void, size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _calloc_internal(n: usize, size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _zalloc_internal(size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_malloc(size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_realloc(ptr: *mut cty::c_void, size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_calloc(n: usize, size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_zalloc(size: usize) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_create_queue(queue_len: i32, item_size: i32) -> *mut cty::c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_delete_queue(queue: *mut cty::c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _modem_sleep_enter(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _modem_sleep_exit(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _modem_sleep_register(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _modem_sleep_deregister(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_status_get() -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_condition_set(type_: u32, dissatisfy: bool) {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_wifi_request(event: u32, latency: u32, duration: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_wifi_release(event: u32) -> i32 {
    unimplemented!()
}
