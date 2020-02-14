extern crate jni;
extern crate ffi;

mod camera_engine;
mod camera_manager;

use jni::JNIEnv;
use jni::objects::{JClass, JValue};
use jni::sys::{jint, jlong, jobject};

use camera_engine::CameraAppEngine;

fn app_engine_create<'a>(env: &'a JNIEnv<'static>, width: i32, height: i32) -> *mut CameraAppEngine<'a> {
    let engine = CameraAppEngine::new(env, width, height);
    Box::into_raw(Box::new(engine))
}

/// Destroys an AppEngine object previously constructed using `rust_engine_create()`.
fn app_engine_destroy(engine: *mut CameraAppEngine) {
    unsafe { drop(Box::from_raw(engine)) }
}


#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_createCamera(env: JNIEnv<'static>, _: JClass, width:jint, height:jint) -> jlong {
    app_engine_create(&env, width, height) as jlong
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_deleteCamera(_: JNIEnv, _: JClass, engine_ptr:jlong) {
    app_engine_destroy(engine_ptr as *mut CameraAppEngine);
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_getMinimumCompatiblePreviewSize(env: JNIEnv, _: JClass, engine_ptr:jlong) -> jobject {

    let app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
    let args: [JValue; 2] = [
        JValue::from(app.get_compatible_camera_res().width), 
        JValue::from(app.get_compatible_camera_res().height)
    ];
        
    let cls = env.find_class("android/util/Size").unwrap();
    env.new_object(cls, "(II)V", &args)
        .unwrap()
        .into_inner()
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceCreated(_: JNIEnv, _: JClass, engine_ptr:jlong, surface:jobject) {
    // let mut app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
    // app.create_camera_session(surface);
    // app.start_preview(true);
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceDestroyed(_: JNIEnv, _: JClass, engine_ptr:jlong, _:jobject) {
    let mut app = unsafe { Box::from_raw(engine_ptr as *mut CameraAppEngine) };
    app.start_preview(false);

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


