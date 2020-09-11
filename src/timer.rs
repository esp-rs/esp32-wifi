use alloc::collections::binary_heap::BinaryHeap;
use core::cell::Cell;
use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use esp32_hal::prelude::*;
use esp32_hal::timer::{self, TimerWithInterrupt};

pub trait Callback {
    fn handle(&self);
}

struct Event<'a, Time: Ord> {
    next: Time,
    interval: Time,
    id: usize,
    callback: &'a dyn Callback,
}

#[derive(Copy, Clone)]
pub struct TimerID {
    id: usize,
}

impl<T: Ord> Ord for Event<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next.cmp(&other.next).reverse()
    }
}

impl<T: Ord> PartialOrd for Event<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.next.cmp(&other.next).reverse())
    }
}

impl<T: Ord> Eq for Event<'_, T> {}

impl<T: Ord> PartialEq for Event<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next
    }
}

pub struct TimerInterruptHandler<'a, TIMER: TimerWithInterrupt> {
    timer_factory: Cell<*mut TimerFactoryImpl<'a, TIMER>>,
}

unsafe impl<'a, TIMER: TimerWithInterrupt> Sync for TimerInterruptHandler<'a, TIMER> {}

impl<'a, TIMER: TimerWithInterrupt> TimerInterruptHandler<'a, TIMER> {
    pub const fn new() -> Self {
        Self {
            timer_factory: Cell::new(core::ptr::null_mut()),
        }
    }

    pub fn handle(&self) {
        unsafe {
            if let Some(timer_factory) = self.timer_factory.get().as_mut() {
                timer_factory.handle_interrupt();
            }
        }
    }

    pub fn set_timer_factory(&self, timer_factory: &mut TimerFactoryImpl<'a, TIMER>) {
        self.timer_factory.set(timer_factory as *mut _);
        timer_factory.set_interrupt_handler(self);
    }

    fn unset_timer_factory(&self) {
        self.timer_factory.set(core::ptr::null_mut());
    }
}

impl<'a, TIMER: TimerWithInterrupt> Drop for TimerInterruptHandler<'a, TIMER> {
    fn drop(&mut self) {
        unsafe {
            if let Some(timer_factory) = self.timer_factory.get().as_mut() {
                timer_factory.unset_interrupt_handler();
            }
        }
    }
}

pub trait TimerFactory<'a> {
    fn add_single(&mut self, time: NanoSecondsU64, callback: &'a dyn Callback) -> TimerID;
    fn add_periodic(
        &mut self,
        time: NanoSecondsU64,
        period: NanoSecondsU64,
        callback: &'a dyn Callback,
    ) -> TimerID;
    fn cancel(&mut self, id: TimerID);
}

pub struct TimerFactoryImpl<'a, TIMER: TimerWithInterrupt> {
    timer: Option<TIMER>,
    queue: BinaryHeap<Event<'a, NanoSecondsU64>>,
    timer_interrupt_handler: Cell<*const TimerInterruptHandler<'a, TIMER>>,
    id: usize,
}

impl<'a, TIMER: TimerWithInterrupt> TimerFactoryImpl<'a, TIMER> {
    pub fn new(mut timer: TIMER) -> Self {
        timer
            .enable(false)
            .set_divider(3)
            .unwrap()
            .auto_reload(false)
            .set_value(0)
            .set_alarm(0)
            .enable_alarm(true)
            .enable(true);

        timer.clear_interrupt();
        timer.listen(timer::Event::TimeOut);

        TimerFactoryImpl {
            timer: Some(timer),
            queue: BinaryHeap::new(),
            timer_interrupt_handler: Cell::new(core::ptr::null()),
            id: 0,
        }
    }

    pub fn release(mut self) -> TIMER {
        self.timer.take().unwrap()
    }

    fn handle_interrupt(&mut self) {
        // TODO: safe handling of queue: wrap in Mutex?
        let timer = self.timer.as_mut().unwrap();

        loop {
            if let Some(event) = self.queue.peek() {
                let now = timer.get_value_in_ns();
                if event.next <= now {
                    event.callback.handle();
                    if let Some(mut event) = self.queue.pop() {
                        if event.interval > NanoSecondsU64(0) {
                            event.next = event.next + event.interval;
                            self.queue.push(event);
                        }
                    }
                }
            }

            if let Some(next_event) = self.queue.peek() {
                timer.set_alarm_in_ns(next_event.next);
                timer.clear_interrupt();
                if next_event.next > timer.get_value_in_ns() {
                    break;
                }
            } else {
                timer.clear_interrupt();
                break;
            }
        }
    }

    fn set_interrupt_handler(&self, handler: &TimerInterruptHandler<'a, TIMER>) {
        self.timer_interrupt_handler.set(handler as *const _);
    }

    fn unset_interrupt_handler(&mut self) {
        self.timer_interrupt_handler.set(core::ptr::null());
    }
}

impl<'a, TIMER: TimerWithInterrupt> TimerFactory<'a> for TimerFactoryImpl<'a, TIMER> {
    fn add_single(&mut self, time: NanoSecondsU64, callback: &'a dyn Callback) -> TimerID {
        self.add_periodic(time, NanoSecondsU64(0), callback)
    }

    fn add_periodic(
        &mut self,
        time: NanoSecondsU64,
        period: NanoSecondsU64,
        callback: &'a dyn Callback,
    ) -> TimerID {
        // TODO: safe handling of queue: wrap in Mutex?

        let now = self.timer.as_ref().unwrap().get_value_in_ns();
        self.id += 1;
        let event = Event {
            next: time + now,
            interval: period.into(),
            id: self.id,
            callback: callback,
        };
        self.queue.push(event);
        self.handle_interrupt();

        TimerID { id: self.id }
    }

    fn cancel(&mut self, id: TimerID) {
        self.queue.retain(|x| x.id != id.id);
        self.handle_interrupt();
    }
}

impl<'a, TIMER: TimerWithInterrupt> Drop for TimerFactoryImpl<'a, TIMER> {
    fn drop(&mut self) {
        unsafe {
            if let Some(timer) = &mut self.timer {
                timer.enable_alarm(false);
                timer.enable(false);
                timer.clear_interrupt();
            }

            if let Some(handler) = self.timer_interrupt_handler.get().as_ref() {
                handler.unset_timer_factory();
            }
        }
    }
}
