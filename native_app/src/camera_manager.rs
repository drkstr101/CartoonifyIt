use std::collections::HashMap;

use ffi::{
    ACameraDevice_request_template,
    ACameraOutputTarget,
    ACaptureRequest,
    ACaptureSessionOutput,
    ANativeWindow,
    AIMAGE_FORMATS, 
    AIMAGE_FORMATS_AIMAGE_FORMAT_YUV_420_888
};

#[allow(dead_code)]
pub enum CaptureSessionState {
    Ready,      // session is ready
    Active,     // session is busy
    Closed,     // session is closed(by itself or a new session evicts)
    MaxState
}

/// A Data Structure to communicate resolution between camera and ImageReader
pub struct ImageFormat {
    pub width: i32,
    pub height: i32,
    // Through out this demo, the format is fixed to
    // YUV_420 format
    pub format: AIMAGE_FORMATS
}

impl ImageFormat {
    pub fn new(width: i32, height: i32) -> ImageFormat {
        let format = AIMAGE_FORMATS_AIMAGE_FORMAT_YUV_420_888;
        ImageFormat{width, height, format}
    }
}

enum PreviewIndices {
    #[allow(dead_code)]
    PreviewRequestIdx = 0,

    #[allow(dead_code)]
    JpgCaptureRequestIdx = 1,

    CaptureRequestCount = 2,
}

struct CaptureRequestInfo {
    pub output_native_window: *mut ANativeWindow,
    pub session_output: *mut ACaptureSessionOutput,
    pub target: *mut ACameraOutputTarget,
    pub request: *mut ACaptureRequest,
    pub template: ACameraDevice_request_template,
    pub session_sequence_id: i32
}

struct CameraId {}

pub struct NDKCamera {
    _requests: Vec<CaptureRequestInfo>,
    _cameras: HashMap<String, CameraId>
}

impl NDKCamera {
    pub fn new() -> NDKCamera {
        let _requests: Vec<CaptureRequestInfo> = 
            Vec::with_capacity(PreviewIndices::CaptureRequestCount as usize);

        let _cameras = HashMap::new();
        NDKCamera { 
            _requests,
            _cameras
        } 
    }
    // pub fn enumerate_camera(&self) {}
    pub fn match_capture_size_request(&self, _: i32, _: i32, _: &ImageFormat) -> bool { true }
    pub fn create_session(&self, _: *mut ANativeWindow) -> bool { true }

    // pub fn get_sensor_orientation(&self, facing: i32, angle: i32) -> bool { true }
    // pub fn on_camera_status_changed(&self, id: (/* `const char*` */), available: bool) { }
    // pub fn on_device_state(&self, dev: &ACameraDevice) { }
    // pub fn on_device_error(&self, dev: &ACameraDevice, err: i32) { }
    // pub fn on_session_state(&self, ses: &ACameraCaptureSession, state: CaptureSessionState) { }
    // pub fn on_capture_sequence_end(&self, session: ACameraCaptureSession, sequence_id: i32, frame_num: i64) { }
    // pub fn on_capture_failed(&self, session: &ACameraCaptureSession, request: &ACaptureRequest, failure: &ACameraCaptureFailure) { }

    pub fn start_preview(&self, _: bool) { }
    // pub fn take_photo(&self) -> bool { true }
    // pub fn get_exposure_range(&self, min: &i64, max: &i64, cur_val: &i64) -> bool { true }
    // pub fn get_sensitivity_range(&self, min: &i64, max: &i64, cur_val: &i64) -> bool { true }

    // pub fn update_camera_request_parameter(&self, code: i32, val: i64) { }
}
