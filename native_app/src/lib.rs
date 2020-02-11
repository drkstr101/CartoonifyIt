
mod log;

use std::os::raw::{c_void};


 #[no_mangle]
 pub unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    saved_state: *mut c_void,
    saved_state_size: usize,
 ) {
    native_app_glue_onCreate(activity, saved_state, saved_state_size);
 }

#[no_mangle]
extern "C" fn android_main(state: *mut c_void) {
    log::info(&format!("android_main({:?})", state));
}

#[link(name = "android")]
extern "C" {
    #[allow(non_snake_case)]
    fn native_app_glue_onCreate(
        activity: *mut c_void,
        saved_state: *mut c_void,
        saved_state_size: usize,
    );
}
