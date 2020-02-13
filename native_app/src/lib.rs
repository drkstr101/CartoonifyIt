extern crate jni;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jint, jlong, jobject};


#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_createCamera(_: JNIEnv, _: JClass, _:jint, _:jint) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_deleteCamera(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_getMinimumCompatiblePreviewSize(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_getCameraSensorOrientation(_: JNIEnv, _: JClass, _:jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_onPreviewSurfaceCreated(_: JNIEnv, _: JClass, _:jlong, _:jobject) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sample_textureview_ViewActivity_onPreviewSurfaceDestroyed(_: JNIEnv, _: JClass, _:jint, _:jint) {}


