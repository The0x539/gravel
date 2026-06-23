use core::ffi::{c_int, c_void};

use gravel_sys::foundation::storage as sys;

use crate::util::SysResult;

fn result(ret: c_int) -> Result<usize> {
    if ret < 0 {
        sys::StatusCode(ret).into_nice()?;
    }
    Ok(ret as usize)
}

pub fn exists(key: u32) -> bool {
    unsafe { sys::persist_exists(key) }
}

pub fn get_size(key: u32) -> Result<usize> {
    let ret = unsafe { sys::persist_get_size(key) };
    result(ret)
}

pub fn read_bool(key: u32) -> bool {
    unsafe { sys::persist_read_bool(key) }
}

pub fn read_int(key: u32) -> i32 {
    unsafe { sys::persist_read_int(key) }
}

pub fn read_bytes(key: u32, buffer: &mut [u8]) -> Result<usize> {
    let (ptr, len) = (buffer.as_mut_ptr().cast::<c_void>(), buffer.len());
    let ret = unsafe { sys::persist_read_data(key, ptr, len) };
    result(ret)
}

/*
pub fn read_string(key: u32, buffer: &mut [u8]) -> Result<usize> {
    let (ptr, len) = (buffer.as_mut_ptr().cast::<c_char>(), buffer.len());
    let ret = unsafe { sys::persist_read_string(key, ptr, len) };
    result(ret)
}
*/

pub fn write_bool(key: u32, value: bool) -> Result<usize> {
    let ret = unsafe { sys::persist_write_bool(key, value) };
    result(ret.0)
}

pub fn write_int(key: u32, value: i32) -> Result<usize> {
    let ret = unsafe { sys::persist_write_int(key, value) };
    result(ret.0)
}

pub fn write_bytes(key: u32, buffer: &[u8]) -> Result<usize> {
    let (ptr, len) = (buffer.as_ptr().cast::<c_void>(), buffer.len());
    let ret = unsafe { sys::persist_write_data(key, ptr, len) };
    result(ret)
}

/*
pub fn write_string(key: u32, buffer: &CStr) -> Result<usize> {
    let ret = unsafe { sys::persist_write_string(key, buffer.as_ptr()) };
    result(ret)
}
*/

pub fn delete(key: u32) -> Result<()> {
    let ret = unsafe { sys::persist_delete(key) };
    result(ret.0).map(drop)
}

pub fn get_max_size() -> usize {
    unsafe { sys::persist_get_max_size() }
}

pub type Result<T> = core::result::Result<T, Error>;

error_enum! {
    pub enum Error: sys::StatusCode;
    /// An error occurred (no description).
    Error = E_ERROR;
    /// No idea what went wrong.
    Unknown = E_UNKNOWN;
    /// There was a generic internal logic error.
    Internal = E_INTERNAL;
    /// The function was not called correctly.
    InvalidArgument = E_INVALID_ARGUMENT;
    /// Insufficient allocatable memory available.
    OutOfMemory = E_OUT_OF_MEMORY;
    /// Insufficient long-term storage available.
    OutOfStorage = E_OUT_OF_STORAGE;
    /// Insufficient resources available.
    OutOfResources = E_OUT_OF_RESOURCES;
    /// Argument out of range (may be dynamic).
    Range = E_RANGE;
    /// Target of operation does not exist.
    DoesNotExist = E_DOES_NOT_EXIST;
    /// Operation not allowed (may depend on state).
    InvalidOperation = E_INVALID_OPERATION;
    /// Another operation prevented this one.
    Busy = E_BUSY;
    /// Operation not completed; try again.
    Again = E_AGAIN;

    @@other = 99999;
}
