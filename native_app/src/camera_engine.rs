
use jni::JNIEnv;

use camera_manager::NDKCamera;

pub struct CameraAppEngine<'a> {
    _env: &'a JNIEnv<'a>,
    _width: i32,
    _height: i32,
    _camera: Box<NDKCamera>
}

impl<'a> CameraAppEngine<'a> {
    pub fn new(_env: &'a JNIEnv<'a>, _width: i32, _height: i32) -> CameraAppEngine {
        let _camera = Box::new(NDKCamera::new());
        CameraAppEngine { _env, _width, _height, _camera }
    }

    pub fn width(&'a self) -> i32 { self._width }

    pub fn height(&'a self) -> i32 { self._height }
}