extern crate android_ndk;
extern crate android_ndk_sys;

mod glue;
mod log;

use std::os::raw::{c_void};

use android_ndk::android_app::AndroidApp;

fn main() {
    log::info("main()");
    let app = unsafe { AndroidApp::from_ptr(glue::get_android_app()) };

    log::info(&format!("app = {:?}", app))
}


 #[no_mangle]
 pub unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    saved_state: *mut c_void,
    saved_state_size: usize,
 ) {
    native_app_glue_onCreate(activity, saved_state, saved_state_size);
 }

#[no_mangle]
extern "C" fn android_main(_: *mut ()) { main(); }

#[link(name = "android")]
extern "C" {
    #[allow(non_snake_case)]
    fn native_app_glue_onCreate(
        activity: *mut c_void,
        saved_state: *mut c_void,
        saved_state_size: usize,
    );
}
