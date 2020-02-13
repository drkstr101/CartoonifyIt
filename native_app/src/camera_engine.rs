
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

    pub fn width(&'a self) -> i32 { self._width }

    pub fn height(&'a self) -> i32 { self._height }
}