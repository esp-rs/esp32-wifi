#![allow(unused_variables)]

use crate::binary::wifi::__va_list_tag;
use crate::timer::TimerID;
use crate::{fwprintln, wprintln};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::raw_vec::RawVec;
use cty::c_void;
use esp32_hal::alloc::{Allocator, DEFAULT_ALLOCATOR, DRAM_ALLOCATOR};
use esp32_hal::prelude::*;

static mut ALLOCATIONS: CriticalSectionSpinLockMutex<BTreeMap<*mut c_void, RawVec<u8, Allocator>>> =
    CriticalSectionSpinLockMutex::new(BTreeMap::new());

const TRUE: i32 = 1;
const FALSE: i32 = 0;

const PASS: i32 = TRUE;
const ESP_OK: i32 = 0;

struct Queue {
    wifi_queue: WifiStaticQueue,
    item_size: usize,
    count: usize,
    send_index: usize,
    receive_index: usize,
}

#[repr(C)]
struct WifiStaticQueue {
    handle: *mut c_void,
    storage: *mut c_void,
}

#[repr(C)]
struct SpinLock {
    owner: u32,
    count: u32,
}
const SPINLOCK_FREE: u32 = 0xB33FFFFF;

impl SpinLock {
    fn new() -> Self {
        Self {
            owner: SPINLOCK_FREE,
            count: 0,
        }
    }
}

struct RecursiveMutex {
    count: CriticalSectionSpinLockMutex<u32>,
}

impl RecursiveMutex {
    const fn new() -> Self {
        Self {
            count: CriticalSectionSpinLockMutex::new(0),
        }
    }

    fn lock(&self) -> u32 {
        (&self.count).lock(|count| {
            *count += 1;
            *count
        })
    }
    fn unlock(&self) -> u32 {
        (&self.count).lock(|count| {
            assert!(*count > 0);
            *count -= 1;
            *count
        })
    }
}

struct Semaphore {
    count: CriticalSectionSpinLockMutex<u32>,
    max: u32,
}

impl Semaphore {
    const fn new(max: u32, initial: u32) -> Self {
        Self {
            max,
            count: CriticalSectionSpinLockMutex::new(initial),
        }
    }

    fn give(&self) -> bool {
        (&self.count).lock(|count| {
            if *count < self.max {
                *count += 1;
                true
            } else {
                false
            }
        })
    }
    fn take(&self) -> bool {
        (&self.count).lock(|count| {
            if *count > 0 {
                *count -= 1;
                true
            } else {
                false
            }
        })
    }
}

static mut QUEUES: CriticalSectionSpinLockMutex<
    BTreeMap<*mut c_void, Box<CriticalSectionSpinLockMutex<Queue>>>,
> = CriticalSectionSpinLockMutex::new(BTreeMap::new());

static mut SEMAPHORES: CriticalSectionSpinLockMutex<BTreeMap<*mut c_void, Box<Semaphore>>> =
    CriticalSectionSpinLockMutex::new(BTreeMap::new());

static mut MUTEXES: CriticalSectionSpinLockMutex<BTreeMap<*mut c_void, Box<RecursiveMutex>>> =
    CriticalSectionSpinLockMutex::new(BTreeMap::new());

static mut SPINLOCKS: CriticalSectionSpinLockMutex<BTreeMap<*mut c_void, Box<SpinLock>>> =
    CriticalSectionSpinLockMutex::new(BTreeMap::new());

static mut TIMERS: CriticalSectionSpinLockMutex<BTreeMap<*mut c_void, Box<Timer>>> =
    CriticalSectionSpinLockMutex::new(BTreeMap::new());

fn alloc(size: usize, internal: bool, zero: bool) -> *mut c_void {
    let memory = RawVec::with_capacity_in(
        size,
        if internal {
            DRAM_ALLOCATOR
        } else {
            DEFAULT_ALLOCATOR
        },
    );
    if zero {
        unsafe { core::ptr::write_bytes(memory.ptr(), 0, size) };
    }
    let address = memory.ptr() as *mut c_void;

    unsafe {
        (&ALLOCATIONS).lock(|allocations| allocations.insert(address, memory));
    }

    wprintln!(
        "alloc({}, {}, {}) -> {:x}",
        size,
        internal,
        zero,
        address as u32
    );
    address
}

pub(crate) static mut WIFI_OS_FUNCS: crate::binary::wifi::wifi_osi_funcs_t =
    crate::binary::wifi::wifi_osi_funcs_t {
        _version: 0x00000007,
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
        _phy_update_country_info: Some(_phy_update_country_info),
        _read_mac: Some(_read_mac),
        _timer_arm: Some(_timer_arm),
        _timer_disarm: Some(_timer_disarm),
        _timer_done: Some(_timer_done),
        _timer_setfn: Some(_timer_setfn),
        _timer_arm_us: Some(_timer_arm_us),
        _wifi_reset_mac: Some(_wifi_reset_mac),
        _wifi_clock_enable: Some(_wifi_clock_enable),
        _wifi_clock_disable: Some(_wifi_clock_disable),
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
        _is_from_isr: Some(_is_from_isr),
        _magic: 0xDEADBEAFu32 as i32,
    };

static mut WIFI_MAC_INTR_HANDLER: Option<(unsafe extern "C" fn(*mut c_void), *mut c_void)> = None;
static mut WIFI_MAC_NMI_HANDLER: Option<(unsafe extern "C" fn(*mut c_void), *mut c_void)> = None;
static mut WIFI_BB_INTR_HANDLER: Option<(unsafe extern "C" fn(*mut c_void), *mut c_void)> = None;

#[interrupt(WIFI_MAC_INTR)]
unsafe fn wifi_mac_intr() {
    wprintln!("WIFI_MAC_INTR fired");
    if let Some((handler, arg)) = WIFI_MAC_INTR_HANDLER {
        handler(arg);
    }
    wprintln!("WIFI_MAC_INTR ISR exiting");
}

#[interrupt(WIFI_MAC_NMI)]
unsafe fn wifi_mac_nmi() {
    wprintln!("WIFI_MAC_NMI fired");
    if let Some((handler, arg)) = WIFI_MAC_NMI_HANDLER {
        handler(arg);
    }
    wprintln!("WIFI_MAC_NMI ISR exiting");
}

#[interrupt(WIFI_BB_INTR)]
unsafe fn wifi_bb_intr() {
    wprintln!("WIFI_BB_INTR fired");
    if let Some((handler, arg)) = WIFI_BB_INTR_HANDLER {
        handler(arg);
    }
    wprintln!("WIFI_BB_INTR ISR exiting");
}

pub unsafe extern "C" fn _set_isr(n: i32, f: *mut c_void, arg: *mut c_void) {
    use esp32_hal::interrupt::Interrupt;
    wprintln!("_set_isr({}, {:x}, {:x})", n, f as u32, arg as u32);

    let handler = core::mem::transmute::<_, unsafe extern "C" fn(*mut c_void)>(f);

    match Interrupt::try_from(n as u8) {
        Ok(Interrupt::WIFI_MAC_INTR) => WIFI_MAC_INTR_HANDLER = Some((handler, arg)),
        Ok(Interrupt::WIFI_MAC_NMI) => WIFI_MAC_NMI_HANDLER = Some((handler, arg)),
        Ok(Interrupt::WIFI_BB_INTR) => WIFI_BB_INTR_HANDLER = Some((handler, arg)),
        Ok(_) => wprintln!("WARNING: didn't expect to _set_isr() to be called for interrupt {}", n),
        Err(_) => wprintln!("WARNING: _set_isr() called with unknown interrupt {}", n),
    }

}
pub unsafe extern "C" fn _ints_on(mask: u32) {
    wprintln!("_ints_on({:x})", mask);
    xtensa_lx6::interrupt::enable_mask(mask);
}
pub unsafe extern "C" fn _ints_off(mask: u32) {
    wprintln!("_ints_off({:x})", mask);
    xtensa_lx6::interrupt::disable_mask(mask);
}
pub unsafe extern "C" fn _spin_lock_create() -> *mut c_void {
    //    unimplemented!()
    let mut spinlock = Box::new(SpinLock::new());
    let address = &mut *spinlock as *mut _ as *mut c_void;

    (&SPINLOCKS).lock(|spinlocks| spinlocks.insert(address, spinlock));
    wprintln!("_spin_lock_create() -> {:x}", address as u32);
    address
}

pub unsafe extern "C" fn _spin_lock_delete(lock: *mut c_void) {
    wprintln!("_spin_lock_delete");
    (&SPINLOCKS).lock(|spinlocks| spinlocks.remove(&lock));
    // unimplemented!()
}

static mut INT_MASK: u32 = 0;

pub unsafe extern "C" fn _wifi_int_disable(wifi_int_mux: *mut c_void) -> u32 {
    wprintln!("_wifi_int_disable({:x?})", wifi_int_mux as u32);

    // disable interrupts and store old mask
    INT_MASK = xtensa_lx6::interrupt::disable();

    0
    //unimplemented!()
}

pub unsafe extern "C" fn _wifi_int_restore(wifi_int_mux: *mut c_void, tmp: u32) {
    wprintln!("_wifi_int_restore({:x?}, {:x?})", wifi_int_mux as u32, tmp);

    // enable previously disable interrupts
    xtensa_lx6::interrupt::enable_mask(INT_MASK);

    //unimplemented!()
}
pub unsafe extern "C" fn _task_yield_from_isr() {
    unimplemented!()
}

fn create_mutex() -> *mut c_void {
    let mut mutex = Box::new(RecursiveMutex::new());
    let address = &mut *mutex as *mut _ as *mut c_void;

    unsafe { (&MUTEXES).lock(|mutexes| mutexes.insert(address, mutex)) };

    wprintln!("create_mutex() -> {:x}", address as u32);
    address
}

fn create_semaphore(max: u32, init: u32) -> *mut c_void {
    let mut semaphore = Box::new(Semaphore::new(max, init));
    let address = &mut *semaphore as *mut _ as *mut c_void;

    unsafe { (&SEMAPHORES).lock(|semaphores| semaphores.insert(address, semaphore)) };

    wprintln!(
        "create_semaphore({}, {}) -> {:x}",
        max,
        init,
        address as u32
    );
    address
}

pub unsafe extern "C" fn _semphr_create(max: u32, init: u32) -> *mut c_void {
    let address = create_semaphore(max, init);
    wprintln!("_semphr_create({}, {}) -> {:x}", max, init, address as u32);
    address
}

pub unsafe extern "C" fn _semphr_delete(semphr: *mut c_void) {
    wprintln!("_semphr_delete({:x?})", semphr as u32);

    (&SEMAPHORES).lock(|semaphores| semaphores.remove(&semphr));
}
pub unsafe extern "C" fn _semphr_take(semphr: *mut c_void, block_time_tick: u32) -> i32 {
    wprintln!("_semphr_take({:x?}, {})", semphr as u32, block_time_tick);

    if block_time_tick == 0 {
        (&SEMAPHORES).lock(|semaphores| {
            let semaphore = semaphores.get(&semphr).unwrap();
            if semaphore.take() {
                wprintln!("_semphr_take -> {}", TRUE);
                return TRUE;
            } else {
                wprintln!("_semphr_take -> {}", FALSE);
                return FALSE;
            }
        });
    };

    loop {
        if (&SEMAPHORES).lock(|semaphores| {
            let semaphore = semaphores.get(&semphr).unwrap();
            semaphore.take()
        }) {
            wprintln!("_semphr_take -> {}", TRUE);
            return TRUE;
        }
    }
}
pub unsafe extern "C" fn _semphr_give(semphr: *mut c_void) -> i32 {
    wprintln!("_semphr_give({:x?})", semphr as u32);

    (&SEMAPHORES).lock(|semaphores| {
        let semaphore = semaphores.get(&semphr).unwrap();
        semaphore.give();
    });
    PASS
}

static mut THREAD_SEMAPHORE_APP: Option<*mut c_void> = None;
static mut THREAD_SEMAPHORE_PRO: Option<*mut c_void> = None;

pub unsafe extern "C" fn _wifi_thread_semphr_get() -> *mut c_void {
    let semaphore = match esp32_hal::get_core() {
        esp32_hal::Core::APP => &mut THREAD_SEMAPHORE_APP,
        esp32_hal::Core::PRO => &mut THREAD_SEMAPHORE_PRO,
    };
    let address = match semaphore {
        None => {
            let address = create_semaphore(1, 0);
            *semaphore = Some(address);
            address
        }
        Some(address) => *address,
    };

    wprintln!("_wifi_thread_semphr_get -> {:x}", address as u32);
    address
    // unimplemented!()
}
pub unsafe extern "C" fn _mutex_create() -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _recursive_mutex_create() -> *mut c_void {
    //    unimplemented!()
    wprintln!("_recursive_mutex_create");
    create_mutex()
}
pub unsafe extern "C" fn _mutex_delete(mutex: *mut c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _mutex_lock(mutex: *mut c_void) -> i32 {
    wprintln!("_mutex_lock ({:x?})", mutex);

    (&MUTEXES).lock(|mutexes| {
        let mutex = mutexes.get(&mutex).unwrap();
        mutex.lock();
    });
    //    unimplemented!()
    TRUE
}
pub unsafe extern "C" fn _mutex_unlock(mutex: *mut c_void) -> i32 {
    //    unimplemented!()
    wprintln!("_mutex_unlock ({:x?})", mutex);
    (&MUTEXES).lock(|mutexes| {
        let mutex = mutexes.get(&mutex).unwrap();
        mutex.unlock();
    });
    TRUE
}
pub unsafe extern "C" fn _queue_create(queue_len: u32, item_size: u32) -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_delete(queue: *mut c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send(
    queue: *mut c_void,
    item: *mut c_void,
    block_time_tick: u32,
) -> i32 {
    wprintln!(
        "_queue_send({:x?}, {:x?}, {})",
        queue,
        item,
        block_time_tick,
    );

    let ticks_start = xtensa_lx6::timer::get_cycle_count();

    loop {
        let res = (&QUEUES).lock(|queues| {
            let queue = (*queues).get(&queue).unwrap();

            (&(**queue)).lock(|queue| {
                if (queue.send_index + queue.count) % queue.count
                    == (queue.receive_index + queue.count - 1) % queue.count
                {
                    FALSE
                } else {
                    core::ptr::copy(
                        item,
                        queue
                            .wifi_queue
                            .storage
                            .add(queue.send_index * queue.item_size),
                        queue.item_size,
                    );
                    queue.send_index = (queue.send_index + 1) % queue.count;
                    wprintln!(
                        "_queue_send {:08x} {:08x} {} -> TRUE",
                        *(item as *mut u32),
                        *((item as u32 + 4) as *mut u32),
                        queue.item_size
                    );
                    TRUE
                }
            })
        });

        if res == TRUE || xtensa_lx6::timer::get_cycle_count() - ticks_start >= block_time_tick {
            wprintln!("_queue_send -> {}", res);

            return res;
        }
    }
}

pub unsafe extern "C" fn _queue_send_from_isr(
    queue: *mut c_void,
    item: *mut c_void,
    hptw: *mut c_void,
) -> i32 {
    wprintln!("WARNING: _queue_send_from_isr() using non-isr variant!");
    let ret = _queue_send(queue, item, u32::MAX);
    if !hptw.is_null() {
        *core::mem::transmute::<_, *mut cty::c_int>(hptw) = 0;
    }
    ret
}
pub unsafe extern "C" fn _queue_send_to_back(
    queue: *mut c_void,
    item: *mut c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_send_to_front(
    queue: *mut c_void,
    item: *mut c_void,
    block_time_tick: u32,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _queue_recv(
    queue: *mut c_void,
    item: *mut c_void,
    block_time_tick: u32,
) -> i32 {
    wprintln!(
        "_queue_recv({:x?}, {:x?}, {:x?})",
        queue,
        item,
        block_time_tick,
    );

    let ticks_start = xtensa_lx6::timer::get_cycle_count();

    loop {
        let res = (&QUEUES).lock(|queues| {
            let queue = (*queues).get(&queue).unwrap();

            (&(**queue)).lock(|queue| {
                if queue.send_index == queue.receive_index {
                    FALSE
                } else {
                    core::ptr::copy(
                        queue
                            .wifi_queue
                            .storage
                            .add(queue.receive_index * queue.item_size),
                        item,
                        queue.item_size,
                    );
                    queue.receive_index = (queue.receive_index + 1) % queue.count;
                    wprintln!(
                        "_queue_recv {:08x} {:08x} {} -> TRUE",
                        *(item as *mut u32),
                        *((item as u32 + 4) as *mut u32),
                        queue.item_size
                    );
                    TRUE
                }
            })
        });

        if res == TRUE || xtensa_lx6::timer::get_cycle_count() - ticks_start >= block_time_tick {
            wprintln!("_queue_recv -> {}", res);

            return res;
        }
    }
}
pub unsafe extern "C" fn _queue_msg_waiting(queue: *mut c_void) -> u32 {
    wprintln!("_queue_msg_waiting({:x?})", queue,);

    (&QUEUES).lock(|queues| {
        let queue = (*queues).get(&queue).unwrap();

        (&(**queue))
            .lock(|queue| (queue.send_index + queue.count - queue.receive_index) % queue.count)
    }) as u32
}
pub unsafe extern "C" fn _event_group_create() -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_delete(event: *mut c_void) {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_set_bits(event: *mut c_void, bits: u32) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_clear_bits(event: *mut c_void, bits: u32) -> u32 {
    unimplemented!()
}
pub unsafe extern "C" fn _event_group_wait_bits(
    event: *mut c_void,
    bits_to_wait_for: u32,
    clear_on_exit: i32,
    wait_for_all_bits: i32,
    block_time_tick: u32,
) -> u32 {
    unimplemented!()
}

static mut TASK_FUNC: Option<(extern "C" fn(params: *mut c_void), *mut c_void)> = None;

fn cpu1_start() -> ! {
    unsafe {
        if let Some((func, param)) = TASK_FUNC {
            func(param);
        }
    }
    wprintln!("Wifi Task returned!");
    loop {}
}

pub unsafe extern "C" fn _task_create_pinned_to_core(
    task_func: *mut c_void,
    name: *const cty::c_char,
    stack_depth: u32,
    param: *mut c_void,
    prio: u32,
    task_handle: *mut c_void,
    core_id: u32,
) -> i32 {
    wprintln!(
        "_task_create_pinned_to_core({:x}, {}, {}, {}, {:x}, {:x}, {})",
        task_func as u32,
        cstr_core::CStr::from_ptr(name).to_str().unwrap(),
        stack_depth,
        param as u32,
        prio,
        task_handle as u32,
        core_id
    );
    TASK_FUNC = Some((core::mem::transmute(task_func), param));
    *(task_handle as *mut u32) = 2;

    esp32_hal::clock_control::ClockControlConfig {}
        .start_app_core(cpu1_start)
        .unwrap();

    PASS
    //unimplemented!()
}
pub unsafe extern "C" fn _task_create(
    task_func: *mut c_void,
    name: *const cty::c_char,
    stack_depth: u32,
    param: *mut c_void,
    prio: u32,
    task_handle: *mut c_void,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _task_delete(task_handle: *mut c_void) {
    wprintln!("_task_delete({:x})", task_handle as u32);
    // unimplemented!();
}
pub unsafe extern "C" fn _task_delay(tick: u32) {
    wprintln!("_task_delay({})", tick);

    xtensa_lx6::timer::delay(tick);
    //    unimplemented!()
}
pub unsafe extern "C" fn _task_ms_to_tick(ms: u32) -> i32 {
    let ticks = ms.ms() * esp32_hal::clock_control::ClockControlConfig {}.cpu_frequency();
    wprintln!("_task_ms_to_tick({}) -> {}", ms, ticks);
    (ticks / Ticks(1)) as i32
}
pub unsafe extern "C" fn _task_get_current_task() -> *mut c_void {
    // unimplemented!()
    esp32_hal::get_core() as u32 as *mut c_void
}
pub unsafe extern "C" fn _task_get_max_priority() -> i32 {
    wprintln!("_task_get_max_priority");
    1
}
pub unsafe extern "C" fn _malloc(size: u32) -> *mut c_void {
    unimplemented!()
}

pub unsafe extern "C" fn _free(p: *mut c_void) {
    wprintln!("_free({:x?})", p);
    (&ALLOCATIONS).lock(|allocations| allocations.remove(&p));
}

pub unsafe extern "C" fn _event_post(
    event_base: *const cty::c_char,
    event_id: i32,
    event_data: *mut c_void,
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

const CONFIG_ESP32_PHY_MAX_TX_POWER: u8 = 20;

const fn limit(val: u8, low: u8, high: u8) -> u8 {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

const PHY_DEFAULT_CALIBRATION_DATA: crate::binary::phy::esp_phy_calibration_data_t =
    crate::binary::phy::esp_phy_calibration_data_t {
        version: [0; 4],
        mac: [0; 6],
        opaque: [0; 1894],
    };

const PHY_DEFAULT_INIT_DATA: crate::binary::phy::esp_phy_init_data_t =
    crate::binary::phy::esp_phy_init_data_t {
        params: [
            3,
            3,
            0x05,
            0x09,
            0x06,
            0x05,
            0x03,
            0x06,
            0x05,
            0x04,
            0x06,
            0x04,
            0x05,
            0x00,
            0x00,
            0x00,
            0x00,
            0x05,
            0x09,
            0x06,
            0x05,
            0x03,
            0x06,
            0x05,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0xfc,
            0xfc,
            0xfe,
            0xf0,
            0xf0,
            0xf0,
            0xe0,
            0xe0,
            0xe0,
            0x18,
            0x18,
            0x18,
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 78),
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 72),
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 66),
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 60),
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 56),
            limit(CONFIG_ESP32_PHY_MAX_TX_POWER * 4, 40, 52),
            0,
            1,
            1,
            2,
            2,
            3,
            4,
            5,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };

static mut COMMON_CLOCK_DISABLE_TIME: u32 = 0;

static mut PHY_RF_INIT_LOCK: CriticalSectionSpinLockMutex<()> = CriticalSectionSpinLockMutex::new(());
static mut PHY_RF_EN_TS: i64 = 0;

unsafe fn phy_update_wifi_mac_time(en_clock_stopped: bool, now: i64) {
    wprintln!("phy_upate_wifi_mac_time({}, {})", en_clock_stopped, now);
    if en_clock_stopped {
        COMMON_CLOCK_DISABLE_TIME = now as u32;
    } else {
        let diff: u32 = (now as u32) - COMMON_CLOCK_DISABLE_TIME;
        wprintln!("-> esp_wifi_internal_update_mac_time({})", diff);
        crate::binary::wifi::esp_wifi_internal_update_mac_time(diff);
        COMMON_CLOCK_DISABLE_TIME = 0;
    }
}

unsafe fn phy_rf_init(init_data: &crate::binary::phy::esp_phy_init_data_t,
                      mode: crate::binary::phy::esp_phy_calibration_mode_t,
                      calibration_data: &mut crate::binary::phy::esp_phy_calibration_data_t,
                      module: u32) -> u32
{
    (&PHY_RF_INIT_LOCK).lock(|_| {
        PHY_RF_EN_TS = _esp_timer_get_time();
        phy_update_wifi_mac_time(false, PHY_RF_EN_TS);

        _phy_common_clock_enable();
        crate::binary::phy::phy_set_wifi_mode_only(false);

        // TODO: implement modem sleep

        wprintln!("register_chipv7_phy({:?})", mode);
        let result = crate::binary::phy::register_chipv7_phy(init_data, calibration_data, mode);
        wprintln!("register_chipv7_phy() -> {}", result);
        // TODO implement ESP_CAL_DATA_CHECK_FAIL return

        crate::binary::phy::coex_bt_high_prio();
        // TODO: implement software coexistence
    });
    0
}

pub unsafe extern "C" fn _phy_load_cal_and_init(module: u32) {
    wprintln!("_phy_load_cal_and_init({})", module);

    let mut cal_data = PHY_DEFAULT_CALIBRATION_DATA.clone();
    let init_data = &PHY_DEFAULT_INIT_DATA;
    phy_rf_init(init_data, crate::binary::phy::esp_phy_calibration_mode_t::PHY_RF_CAL_FULL, &mut cal_data, module);
}

static mut PHY_COMMON_CLOCK_REF_COUNT: CriticalSectionSpinLockMutex<u8> =
    CriticalSectionSpinLockMutex::new(0);

pub unsafe extern "C" fn _phy_common_clock_enable() {
    wprintln!("_phy_common_clock_enable");

    (&PHY_COMMON_CLOCK_REF_COUNT).lock(|count| {
        if *count == 0 {
            esp32_hal::dport::enable_peripheral(esp32_hal::dport::Peripheral::WIFI_BT_COMMON);
        }
        *count += 1;
    });
}
pub unsafe extern "C" fn _phy_common_clock_disable() {
    wprintln!("_phy_common_clock_disable");

    (&PHY_COMMON_CLOCK_REF_COUNT).lock(|count| {
        *count -= 1;
        if *count == 0 {
            esp32_hal::dport::disable_peripheral(esp32_hal::dport::Peripheral::WIFI_BT_COMMON);
        }
    });
}

pub unsafe extern "C" fn _phy_update_country_info(country: *const cty::c_char) -> i32 {
    wprintln!("WARNING: phy_update_country_info({}) unimplemented", cstr_core::CStr::from_ptr(country).to_str().unwrap());
    0
    //unimplemented!();
}

#[allow(dead_code)]
enum MACType {
    Station = 0,
    AccessPoint = 1,
    Bluetooth = 2,
    Ethernet = 3,
}

pub unsafe extern "C" fn _read_mac(mac: *mut u8, mac_type: u32) -> i32 {
    if mac_type > MACType::Ethernet as u32 {
        wprintln!("_read_mac({:x} , {}) -> FALSE ", mac as u32, mac_type);
        return FALSE;
    }

    let mut efuse_mac = esp32_hal::efuse::Efuse::get_mac_address();
    efuse_mac[5] += mac_type as u8;

    core::ptr::copy(
        &mut efuse_mac as *mut _ as *mut u8,
        mac,
        core::mem::size_of_val(&efuse_mac),
    );

    wprintln!(
        "_read_mac({:x} -> {:x?}, {}) -> TRUE ",
        mac as u32,
        efuse_mac,
        mac_type
    );
    TRUE
}
pub unsafe extern "C" fn _timer_arm(ptimer: *mut c_void, tmout: u32, repeat: bool) {
    wprintln!("_timer_arm({:x}, {}, {})", ptimer as u32, tmout, repeat);

    (&TIMERS).lock(|timers| {
        let timer = timers.get_mut(&ptimer).unwrap();
        if repeat {
            timer.id = Some(
                crate::wifi::WiFi::get_timer_factory().add_periodic(MilliSeconds(tmout).into(), MilliSeconds(tmout).into(), &**timer)
            );
        } else {
            timer.id = Some(
                crate::wifi::WiFi::get_timer_factory().add_single(MilliSeconds(tmout).into(), &**timer),
            );
        }
    });
}

pub unsafe extern "C" fn _timer_disarm(ptimer: *mut c_void) {
    wprintln!("_timer_disarm({:x})", ptimer as u32);

    (&TIMERS).lock(|timers| {
        if let Some(timer) = timers.get_mut(&ptimer) {
            if let Some(id) = timer.id {
                crate::wifi::WiFi::get_timer_factory().cancel(id);
            }
        }
    });
}

pub unsafe extern "C" fn _timer_done(ptimer: *mut c_void) {
    wprintln!("_timer_done({:x?}) ", ptimer);

    (&TIMERS).lock(|timers| {
        if let Some(timer) = timers.get_mut(&ptimer) {
            if let Some(id) = timer.id {
                crate::wifi::WiFi::get_timer_factory().cancel(id);
            }
            timers.remove(&ptimer);
        }
    });
}

struct Timer {
    id: Option<TimerID>,
    pfunction: unsafe extern "C" fn(args: *mut c_void),
    parg: *mut c_void,
}

impl crate::timer::Callback for Timer {
    fn handle(&self) {
        fwprintln!("Timer Callback: {:x}", self.pfunction as u32);
        unsafe { (self.pfunction)(self.parg) };
    }
}

pub unsafe extern "C" fn _timer_setfn(
    ptimer: *mut c_void,
    pfunction: *mut c_void,
    parg: *mut c_void,
) {
    (&TIMERS).lock(|timers| {
        if let Some(timer) = timers.get_mut(&ptimer) {
            timer.pfunction = core::mem::transmute(pfunction);
            timer.parg = parg;
        } else {
            let timer = Box::new(Timer {
                id: None,
                pfunction: core::mem::transmute(pfunction),
                parg,
            });

            timers.insert(ptimer, timer);
        };
    });

    wprintln!("_timer_setfn({:x?}, {:x?}, {:x?})", ptimer, pfunction, parg);
}

pub unsafe extern "C" fn _timer_arm_us(ptimer: *mut c_void, us: u32, repeat: bool) {
    unimplemented!()
}

const DPORT_MAC_RST: u32 = 1 << 2;

pub unsafe extern "C" fn _wifi_reset_mac() {
    wprintln!("wifi_reset_mac()");
    let dport = &(*esp32::DPORT::ptr());
    dport.core_rst_en.modify(|r, w| w.bits(r.bits() | DPORT_MAC_RST));
    dport.core_rst_en.modify(|r, w| w.bits(r.bits() & !DPORT_MAC_RST));
}

static mut WIFI_CLOCK_REF_COUNT: CriticalSectionSpinLockMutex<u8> =
    CriticalSectionSpinLockMutex::new(0);

pub unsafe extern "C" fn _wifi_clock_enable() {
    wprintln!("wifi_clock_enable()");
    (&WIFI_CLOCK_REF_COUNT).lock(|count| {
        if *count == 0 {
            esp32_hal::dport::enable_peripheral(esp32_hal::dport::Peripheral::WIFI);
        }
        *count += 1;
    });
}

pub unsafe extern "C" fn _wifi_clock_disable() {
    wprintln!("wifi_clock_disable()");
    (&WIFI_CLOCK_REF_COUNT).lock(|count| {
        *count -= 1;
        if *count == 0 {
            esp32_hal::dport::disable_peripheral(esp32_hal::dport::Peripheral::WIFI);
        }
    });
}

pub unsafe extern "C" fn _esp_timer_get_time() -> i64 {
    let now = esp32_hal::clock_control::ClockControlConfig{}.rtc_nanoseconds();
    wprintln!("_esp_timer_get_time() -> {}", now);
    (now.0 / 1000) as i64
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
    value: *const c_void,
    length: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _nvs_get_blob(
    handle: u32,
    key: *const cty::c_char,
    out_value: *mut c_void,
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
pub unsafe extern "C" fn _get_time(t: *mut c_void) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _random() -> cty::c_ulong {
    unimplemented!()
}
pub unsafe extern "C" fn _log_write(
    level: u32,
    tag: *const cty::c_char,
    format: *const cty::c_char,
    mut args: ...
) {
    let a: u32 = args.arg();
    wprintln!(
        "_log_write({}, {}, {}, {:x?})",
        level,
        cstr_core::CStr::from_ptr(tag).to_str().unwrap(),
        cstr_core::CStr::from_ptr(format).to_str().unwrap(),
        a
    );
    // unimplemented!()
}
pub unsafe extern "C" fn _log_writev(
    level: u32,
    tag: *const cty::c_char,
    format: *const cty::c_char,
    args: *mut __va_list_tag,
) {
    wprintln!(
        "_log_writev({}, {}, {}, ...)",
        level,
        cstr_core::CStr::from_ptr(tag).to_str().unwrap(),
        cstr_core::CStr::from_ptr(format).to_str().unwrap(),
    );
    //    unimplemented!()
}
pub unsafe extern "C" fn _log_timestamp() -> u32 {
    // unimplemented!()
    1
}

pub unsafe extern "C" fn _malloc_internal(size: usize) -> *mut c_void {
    wprintln!("_malloc_internal({})", size);
    alloc(size, true, false)
}

pub unsafe extern "C" fn _realloc_internal(ptr: *mut c_void, size: usize) -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _calloc_internal(n: usize, size: usize) -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _zalloc_internal(size: usize) -> *mut c_void {
    wprintln!("_zalloc_internal({})", size);
    alloc(size, true, true)
}
pub unsafe extern "C" fn _wifi_malloc(size: usize) -> *mut c_void {
    wprintln!("_wifi_malloc({})", size);
    alloc(size, false, false)
}
pub unsafe extern "C" fn _wifi_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unimplemented!()
}
pub unsafe extern "C" fn _wifi_calloc(n: usize, size: usize) -> *mut c_void {
    wprintln!("_wifi_calloc({})", size);
    alloc(n * size, false, true)
}
pub unsafe extern "C" fn _wifi_zalloc(size: usize) -> *mut c_void {
    wprintln!("_wifi_zalloc({})", size);
    alloc(size, false, true)
}

pub unsafe extern "C" fn _wifi_create_queue(queue_len: i32, item_size: i32) -> *mut c_void {
    let queue = Box::new(CriticalSectionSpinLockMutex::new(Queue {
        wifi_queue: WifiStaticQueue {
            handle: 0 as *mut c_void,
            storage: alloc(queue_len as usize * item_size as usize, true, true),
        },
        count: queue_len as usize,
        item_size: item_size as usize,
        send_index: 0,
        receive_index: 0,
    }));
    let address = (&*queue).lock(|queue| {
        let address = &mut (*queue).wifi_queue as *mut _ as *mut c_void;
        (*queue).wifi_queue.handle = address;
        address
    });
    (&QUEUES).lock(|queues| queues.insert(address, queue));

    wprintln!(
        "_wifi_create_queue({}, {}) -> {:x?}",
        queue_len,
        item_size,
        address
    );
    address
}
pub unsafe extern "C" fn _wifi_delete_queue(queue: *mut c_void) {
    wprintln!("_wifi_delete_queue({:x?})", queue as u32);
    //unimplemented!()
}

pub unsafe extern "C" fn _modem_sleep_enter(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _modem_sleep_exit(module: u32) -> i32 {
    wprintln!("_modem_sleep_exit({})", module);
    //    unimplemented!()
    ESP_OK
}
pub unsafe extern "C" fn _modem_sleep_register(module: u32) -> i32 {
    wprintln!("_modem_sleep_register({})", module);
    //    unimplemented!()
    ESP_OK
}
pub unsafe extern "C" fn _modem_sleep_deregister(module: u32) -> i32 {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_status_get() -> u32 {
    wprintln!("_coex_status_get()");
    // unimplemented!()
    0
}
pub unsafe extern "C" fn _coex_condition_set(type_: u32, dissatisfy: bool) {
    unimplemented!()
}
pub unsafe extern "C" fn _coex_wifi_request(event: u32, latency: u32, duration: u32) -> i32 {
    wprintln!("_coex_wifi_request({}, {}, {})", event, latency, duration);
    // unimplemented!()
    return 0; // use hardware coexistance
}
pub unsafe extern "C" fn _coex_wifi_release(event: u32) -> i32 {
    wprintln!("_coex_wifi_release({})", event);
    // unimplemented!()
    return 0; // use hardware coexistance
}

pub unsafe extern "C" fn _is_from_isr() -> bool {
    unimplemented!();
}
