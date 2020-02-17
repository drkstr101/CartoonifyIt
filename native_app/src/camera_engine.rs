
use ffi::{JNIEnv, jobject};

use camera_manager::{ImageFormat, NDKCamera};

pub struct CameraAppEngine {
    _env: *mut JNIEnv,
    _request_width: i32,
    _request_height: i32,
    _camera: Box<NDKCamera>,
    _compatible_camera_res: Box<ImageFormat>,
    _surface: Option<jobject>
}

impl CameraAppEngine {
    pub fn new(_env: &mut JNIEnv, _request_width: i32, _request_height: i32) -> CameraAppEngine {
        let view = ImageFormat::new(_request_width, _request_height);
        let camera = NDKCamera::new();
        camera.match_capture_size_request(_request_width, _request_height, &view);

        let _camera = Box::new(camera);
        let _compatible_camera_res = Box::new(view);

        let _surface = Some(0 as jobject);
        CameraAppEngine { 
            _env, 
            _request_width,
            _request_height, 
            _camera, 
            _compatible_camera_res,
            _surface
        }
    }

    pub fn get_compatible_camera_res(&self) -> &Box<ImageFormat> { &self._compatible_camera_res }

    pub fn create_camera_session(&mut self, surface: jobject) {
        // error!
        // let window = ffi::ANativeWindow_fromSurface(self._env, surface);
    }

    pub fn start_preview(&mut self, start: bool) {
        self.camera().start_preview(start);
    }

    fn camera(&self) -> &NDKCamera { &self._camera }
}
