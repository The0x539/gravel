use crate::std_c::time::time_t;

unsafe extern "C" {
    pub fn wakeup_service_subscribe(handler: WakeupHandler);
    pub fn wakeup_schedule(timestamp: time_t, cookie: i32, notify_if_missed: bool) -> WakeupId;
    pub fn wakeup_cancel(wakeup_id: WakeupId);
    pub fn wakeup_cancel_all();
    pub fn wakeup_get_launch_event(wakeup_id: *mut WakeupId, cookie: *mut i32) -> bool;
    pub fn wakeup_query(wakeup_id: WakeupId, timestamp: *mut time_t) -> bool;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WakeupId(pub i32);

callbacks! {
    pub fn WakeupHandler(wakeup_id: WakeupId, cookie: i32);
}
