use gravel_sys::foundation::wakeup as sys;
use gravel_sys::std_c::time::time_t;
pub use sys::WakeupId;

pub fn subscribe(handler: WakeupHandler) {
    unsafe { sys::wakeup_service_subscribe(transmute_handler(handler)) }
}

pub fn schedule(timestamp: time_t, cookie: i32, notify_if_missed: bool) -> WakeupId {
    unsafe { sys::wakeup_schedule(timestamp, cookie, notify_if_missed) }
}

pub fn cancel(wakeup_id: WakeupId) {
    unsafe { sys::wakeup_cancel(wakeup_id) }
}

pub fn cancel_all() {
    unsafe { sys::wakeup_cancel_all() }
}

pub fn get_launch_event() -> Option<(WakeupId, i32)> {
    let mut event: (_, _) = Default::default();
    let ret = unsafe { sys::wakeup_get_launch_event(&mut event.0, &mut event.1) };
    ret.then_some(event)
}

pub fn query(wakeup_id: WakeupId) -> Option<time_t> {
    let mut timestamp = Default::default();
    let ret = unsafe { sys::wakeup_query(wakeup_id, &mut timestamp) };
    ret.then_some(timestamp)
}

pub type WakeupHandler = fn(wakeup_id: WakeupId, cookie: i32);

fn transmute_handler(f: WakeupHandler) -> sys::WakeupHandler {
    unsafe { core::mem::transmute(f) }
}
