
use std::ffi::{CString};
use std::os::raw::{c_char, c_int};
use std::error::Error;

const LEVEL_INFO: c_int = 4;
const LEVEL_WARN: c_int = 5;
const LEVEL_ERROR: c_int = 6;

const TAG: &'static [u8] = b"CartoonifyIt";

pub fn info(msg: &str) {
    log(msg, LEVEL_INFO);
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    log(msg, LEVEL_WARN);
}

#[allow(dead_code)]
#[allow(bare_trait_objects)]
pub fn error(e: &Error) {
    log(&format!("{}", e), LEVEL_ERROR)
}

fn log(msg: &str, lvl: c_int) {
    let tag = CString::new(TAG).unwrap();
    let msg = CString::new(msg.as_bytes()).unwrap_or(CString::new("<malformed log message>").unwrap());
    unsafe {
        __android_log_write(lvl, tag.as_ptr(), msg.as_ptr())
    };
}

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}