use std::os::raw::c_char;
use std::ffi::CString;

use ffi::{JNIEnv, jclass, jint, jmethodID, jobject, jvalue};

pub fn str_into_raw(x: &'static str) -> *const c_char {
    CString::new(x)
        .expect("CString::new failed")
        .into_raw()
}

pub fn jint_val(x: i32) -> jvalue {
    jvalue{ i: x as jint }
}

pub mod jni {
    use super::*;

    pub unsafe fn find_class(env: &mut JNIEnv, name: &'static str) -> jclass {
        let func = env.as_ref().unwrap().FindClass.unwrap();
        func(env, str_into_raw(name))
    }

    pub unsafe fn get_method_id(env: &mut JNIEnv, cls: jclass, name: &'static str, sig: &'static str) -> jmethodID {
        let func = env.as_ref().unwrap().GetMethodID.unwrap();
        func(env, cls, str_into_raw(name), str_into_raw(sig))
    }

    pub unsafe fn new_object(env: &mut JNIEnv, cls: jclass, ctor_id: jmethodID, ctor_args: &[jvalue]) -> jobject {
        let func = env.as_ref().unwrap().NewObjectA.unwrap();
        func(env, cls, ctor_id, ctor_args.as_ptr())
    }
}