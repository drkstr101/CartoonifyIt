extern crate ffi;
extern crate log;

mod camera_engine;
mod camera_manager;

mod util;

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate ffi;

    use super::*;
    use self::ffi::{JNIEnv, jclass, jint, jlong, jobject, jvalue};


    use camera_engine::CameraAppEngine;

    fn app_engine_create(env: &mut JNIEnv, width: i32, height: i32) -> *mut CameraAppEngine {
        let engine = CameraAppEngine::new(env, width, height);
        Box::leak(Box::new(engine))
    }

    /// Destroys an AppEngine object previously constructed using `rust_engine_create()`.
    fn app_engine_destroy(engine: *mut CameraAppEngine) {
        unsafe { drop(Box::from_raw(engine)) }
    }

    #[no_mangle]
    pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_createCamera(env: &mut JNIEnv, _: jclass, width:jint, height:jint) -> jlong {
        app_engine_create(env, width, height) as jlong
    }

    #[no_mangle]
    pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_deleteCamera(_: &mut JNIEnv, _: jclass, engine_ptr:jlong) {
        app_engine_destroy(engine_ptr as *mut CameraAppEngine);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_getMinimumCompatiblePreviewSize(env: &mut JNIEnv, _: jclass, engine_ptr:jlong) -> jobject {

        let app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
        let args: [jvalue; 2] = [
            util::jint_val(app.get_compatible_camera_res().width),
            util::jint_val(app.get_compatible_camera_res().height)
        ];

       let cls = util::jni::find_class(env, "android/util/Size");
       let method_id = util::jni::get_method_id(env, cls, "<init>", "(II)V");
       util::jni::new_object(env, cls, method_id, &args)
    }

    #[no_mangle]
    pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceCreated(_: &mut JNIEnv, _: jclass, engine_ptr:jlong, surface:jobject) {
       let mut app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
       app.create_camera_session(surface);
       app.start_preview(true);
    }

    #[no_mangle]
    pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceDestroyed(_: &mut JNIEnv, _: jclass, engine_ptr:jlong, _:jobject) {
//        let mut app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
//        app.start_preview(false);

        // TODO port cpp code if necessary
        // jclass cls = env->FindClass("android/view/Surface");
        // jmethodID toString =
        //     env->GetMethodID(cls, "toString", "()Ljava/lang/String;");

        // jstring destroyObjStr =
        //     reinterpret_cast<jstring>(env->CallObjectMethod(surface, toString));
        // const char *destroyObjName = env->GetStringUTFChars(destroyObjStr, nullptr);

        // jstring appObjStr = reinterpret_cast<jstring>(
        //     env->CallObjectMethod(pApp->GetSurfaceObject(), toString));
        // const char *appObjName = env->GetStringUTFChars(appObjStr, nullptr);

        // ASSERT(!strcmp(destroyObjName, appObjName), "object Name MisMatch");

        // env->ReleaseStringUTFChars(destroyObjStr, destroyObjName);
        // env->ReleaseStringUTFChars(appObjStr, appObjName);
    }


}