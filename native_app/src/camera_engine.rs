
use jni::JNIEnv;
use jni::sys::{jobject};

use camera_manager::{ImageFormat, NDKCamera};

pub struct CameraAppEngine<'a> {
    _env: &'a JNIEnv<'a>,
    _request_width: i32,
    _request_height: i32,
    _camera: Box<NDKCamera>,
    _compatible_camera_res: Box<ImageFormat>
}

impl<'a> CameraAppEngine<'a> {
    pub fn new(_env: &'a JNIEnv<'a>, _request_width: i32, _request_height: i32) -> CameraAppEngine {
        let _camera = Box::new(NDKCamera::new());
        let _compatible_camera_res = Box::new(ImageFormat::new(_request_width, _request_height));
        CameraAppEngine { _env, _request_width, _request_height, _camera, _compatible_camera_res }
    }

    pub fn get_compatible_camera_res(&'a self) -> &Box<ImageFormat> { &self._compatible_camera_res }

    pub fn create_camera_session(&'a self, _: jobject) {}

    pub fn start_preview(&'a self, _: bool) {}
}