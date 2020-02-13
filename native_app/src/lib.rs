extern crate jni;

mod camera_engine;

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
        JValue::from(app.width()), 
        JValue::from(app.height())
    ];
        
    let cls = env.find_class("android/util/Size").unwrap();
    env.new_object(cls, "(II)V", &args)
        .unwrap()
        .into_inner()
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_getCameraSensorOrientation(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceCreated(_: JNIEnv, _: JClass, _:jlong, _:jobject) {}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceDestroyed(_: JNIEnv, _: JClass, _:jlong, _:jobject) {}


