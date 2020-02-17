use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring, jlong};


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
    pub unsafe extern fn Java_io_waweb_cartoonifyit_MainActivity_greeting(env: JNIEnv, _: JClass, app_ptr: jlong) -> jstring {
        let app = app_ptr as *mut AppEngine;
        let greeting_ptr = CString::from_raw(rust_greeting(app));
        let output = env.new_string(greeting_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_io_waweb_cartoonifyit_MainActivity_createNativeApp(env: JNIEnv, _: JClass, java_pattern: JString) -> jlong {
        rust_engine_create(env.get_string(java_pattern).expect("invalid string").as_ptr()) as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_destroyNativeApp(_: JNIEnv, _: JClass, app_ptr: jlong) {
        let app = app_ptr as *mut AppEngine;
        rust_engine_destroy(app)
    }
}
