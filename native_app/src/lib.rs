extern crate jni;

mod camera_engine;

use jni::JNIEnv;
use jni::objects::JClass;
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
    let engine = engine_ptr as *mut CameraAppEngine;
    app_engine_destroy(engine);
}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_getMinimumCompatiblePreviewSize(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_getCameraSensorOrientation(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceCreated(_: JNIEnv, _: JClass, _:jlong, _:jobject) {}

#[no_mangle]
pub extern "C" fn Java_io_waweb_cartoonifyit_MainActivity_onPreviewSurfaceDestroyed(_: JNIEnv, _: JClass, _:jlong, _:jobject) {}


