use std::collections::HashMap;

use ffi::{
    acamera_metadata_enum_android_lens_facing_t,
    acamera_metadata_enum_acamera_lens_facing_ACAMERA_LENS_FACING_FRONT as ACAMERA_LENS_FACING_FRONT,
    ACameraDevice,
    ACameraDevice_request_template,
    ACameraManager,
    ACameraManager_create,
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
    PreviewRequestIdx = 0,
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

struct CameraId {
    pub id: String,
    pub device: *mut ACameraDevice,
    pub facing: acamera_metadata_enum_android_lens_facing_t,
    pub available: bool,
    pub owner: bool
}

impl CameraId {

    #[allow(dead_code)]
    pub fn new(id: String) -> Self {
        CameraId {
            id,
            device: (0 as *mut ACameraDevice),
            facing: ACAMERA_LENS_FACING_FRONT,
            available: false,
            owner: false
        }
    }
}

pub struct NDKCamera {
    _requests: Vec<CaptureRequestInfo>,
    _cameras: HashMap<String, CameraId>,
    _camera_mgr: *mut ACameraManager,
    _active_camera_id: String
}

impl NDKCamera {
    pub fn new() -> NDKCamera {
        let _requests: Vec<CaptureRequestInfo> = 
            Vec::with_capacity(PreviewIndices::CaptureRequestCount as usize);

        let _cameras = HashMap::new();

        let _camera_mgr = unsafe { ACameraManager_create() };
        // assert!(_camera_mgr, "Failed to create cameraManager");

        let _active_camera_id = "".to_string();

        let mut camera = NDKCamera { 
            _requests,
            _cameras,
            _camera_mgr,
            _active_camera_id
        };

        // Pick up a back-facing camera to preview
        camera.enumerate_camera();
        assert!(camera._active_camera_id.len() > 0, "Unknown ActiveCameraIdx");

        // // Create back facing camera device
        // CALL_MGR(openCamera(cameraMgr_, activeCameraId_.c_str(), GetDeviceListener(),
        //                     &cameras_[activeCameraId_].device_));

        // CALL_MGR(registerAvailabilityCallback(cameraMgr_, GetManagerListener()));

        // // Initialize camera controls(exposure time and sensitivity), pick
        // // up value of 2% * range + min as starting value (just a number, no magic)
        // ACameraMetadata* metadataObj;
        // CALL_MGR(getCameraCharacteristics(cameraMgr_, activeCameraId_.c_str(),
        //                                     &metadataObj));
        // ACameraMetadata_const_entry val = {
        //     0,
        // };
        // camera_status_t status = ACameraMetadata_getConstEntry(
        //     metadataObj, ACAMERA_SENSOR_INFO_EXPOSURE_TIME_RANGE, &val);
        // if (status == ACAMERA_OK) {
        //     exposureRange_.min_ = val.data.i64[0];
        //     if (exposureRange_.min_ < kMinExposureTime) {
        //     exposureRange_.min_ = kMinExposureTime;
        //     }
        //     exposureRange_.max_ = val.data.i64[1];
        //     if (exposureRange_.max_ > kMaxExposureTime) {
        //     exposureRange_.max_ = kMaxExposureTime;
        //     }
        //     exposureTime_ = exposureRange_.value(2);
        // } else {
        //     LOGW("Unsupported ACAMERA_SENSOR_INFO_EXPOSURE_TIME_RANGE");
        //     exposureRange_.min_ = exposureRange_.max_ = 0l;
        //     exposureTime_ = 0l;
        // }
        // status = ACameraMetadata_getConstEntry(
        //     metadataObj, ACAMERA_SENSOR_INFO_SENSITIVITY_RANGE, &val);

        // if (status == ACAMERA_OK){
        //     sensitivityRange_.min_ = val.data.i32[0];
        //     sensitivityRange_.max_ = val.data.i32[1];

        //     sensitivity_ = sensitivityRange_.value(2);
        // } else {
        //     LOGW("failed for ACAMERA_SENSOR_INFO_SENSITIVITY_RANGE");
        //     sensitivityRange_.min_ = sensitivityRange_.max_ = 0;
        //     sensitivity_ = 0;
        // }
        // valid_ = true;

        camera
    }
    pub fn enumerate_camera(&mut self) {
        // TODO...

        // ACameraIdList* cameraIds = nullptr;
        // CALL_MGR(getCameraIdList(cameraMgr_, &cameraIds));
      
        // for (int i = 0; i < cameraIds->numCameras; ++i) {
        //   const char* id = cameraIds->cameraIds[i];
      
        //   ACameraMetadata* metadataObj;
        //   CALL_MGR(getCameraCharacteristics(cameraMgr_, id, &metadataObj));
      
        //   int32_t count = 0;
        //   const uint32_t* tags = nullptr;
        //   ACameraMetadata_getAllTags(metadataObj, &count, &tags);
        //   for (int tagIdx = 0; tagIdx < count; ++tagIdx) {
        //     if (ACAMERA_LENS_FACING == tags[tagIdx]) {
        //       ACameraMetadata_const_entry lensInfo = {
        //           0,
        //       };
        //       CALL_METADATA(getConstEntry(metadataObj, tags[tagIdx], &lensInfo));
        //       CameraId cam(id);
        //       cam.facing_ = static_cast<acamera_metadata_enum_android_lens_facing_t>(
        //           lensInfo.data.u8[0]);
        //       cam.owner_ = false;
        //       cam.device_ = nullptr;
        //       cameras_[cam.id_] = cam;
        //       if (cam.facing_ == ACAMERA_LENS_FACING_BACK) {
        //         activeCameraId_ = cam.id_;
        //       }
        //       break;
        //     }
        //   }
        //   ACameraMetadata_free(metadataObj);
    }

    pub fn match_capture_size_request(&self, _: i32, _: i32, _: &ImageFormat) -> bool {
        // TODO...
        true
    }

    pub fn create_session(&self, _: *mut ANativeWindow) -> bool {
        // TODO...
        true
    }

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
