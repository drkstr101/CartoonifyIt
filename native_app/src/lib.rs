use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate ffi;

    use super::*;
    use self::ffi::{JNIEnv, jclass, jstring, jlong};


    #[derive(Debug)]
    struct AppEngine {
        greeting: *mut c_char
    }

    unsafe fn rust_greeting(app: *mut AppEngine) -> *mut c_char {
        let app = Box::from_raw(app);
        app.greeting
    }

    /// Constructs an AppEngine object.
    fn rust_engine_create(to: *const c_char) -> *mut AppEngine {
        let c_str = unsafe { CStr::from_ptr(to) };
        let recipient = match c_str.to_str() {
            Err(_) => "there",
            Ok(string) => string,
        };

        let greeting = CString::new("Hello ".to_owned() + recipient).unwrap().into_raw();

        let app = AppEngine{greeting: greeting};
        Box::into_raw(Box::new(app))
    }

    /// Destroys an AppEngine object previously constructed using `rust_engine_create()`.
    unsafe fn rust_engine_destroy(app: *mut AppEngine) {
        drop(Box::from_raw(app))
    }

    #[no_mangle]
    pub unsafe extern fn Java_io_waweb_cartoonifyit_MainActivity_greeting(_: JNIEnv, _: jclass, app_ptr: jlong) -> jstring {
        let app = app_ptr as *mut AppEngine;
        rust_greeting(app) as jstring
    }

    #[no_mangle]
    pub unsafe extern fn Java_io_waweb_cartoonifyit_MainActivity_createNativeApp(env: &mut JNIEnv, _: jclass, java_pattern: jstring) -> jlong {
        let get_string_chars = env.as_ref().unwrap().GetStringChars.unwrap();
        let is_copy = 0 as *mut u8;
        rust_engine_create(get_string_chars(env, java_pattern, is_copy) as *const c_char ) as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_destroyNativeApp(_: JNIEnv, _: jclass, app_ptr: jlong) {
        let app = app_ptr as *mut AppEngine;
        rust_engine_destroy(app)
    }
}
