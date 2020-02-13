
use jni::JNIEnv;

pub struct CameraAppEngine<'a> {
    _env: &'a JNIEnv<'a>,
    _width: i32,
    _height: i32
}

impl<'a> CameraAppEngine<'a> {
    pub fn new(_env: &'a JNIEnv<'a>, _width: i32, _height: i32) -> CameraAppEngine {
        CameraAppEngine { _env, _width, _height }
    }
}