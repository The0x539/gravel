use gravel_sys::foundation::watch_info as sys;

pub use sys::{
    WatchInfoColor as DeviceColor, WatchInfoModel as DeviceModel,
    WatchInfoVersion as FirmwareVersion,
};

pub fn model() -> DeviceModel {
    unsafe { sys::watch_info_get_model() }
}

pub fn firmware() -> FirmwareVersion {
    unsafe { sys::watch_info_get_firmware_version() }
}

pub fn color() -> DeviceColor {
    unsafe { sys::watch_info_get_color() }
}
