use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;

use ffi::{
    acamera_metadata_enum_android_lens_facing_t,
    acamera_metadata_enum_acamera_lens_facing_ACAMERA_LENS_FACING_FRONT as ACAMERA_LENS_FACING_FRONT,
    ACameraDevice,
    ACameraDevice_request_template,
    ACameraManager,
    ACameraManager_create,
    ACameraManager_openCamera,
    ACameraDevice_ErrorStateCallback,
    ACameraDevice_StateCallback,
    ACameraDevice_StateCallbacks,
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

#[no_mangle]
extern "C" fn on_device_state_changes(ctx: *mut c_void, dev: *mut ACameraDevice) {
    // TODO...
    // let camera = Box::from_raw(ctx);
    // camera.on_device_state(dev);
}

#[no_mangle]
extern "C" fn on_device_error_changes(ctx: *mut c_void, dev: *mut ACameraDevice, err: i32) {
    // TODO...
    // let camera = Box::from_raw(ctx);
    // camera.on_device_error(dev, err);
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

        // Create back facing camera device
        // TODO...
        // let c_str = CString::new(camera._active_camera_id)
        //     .expect("Failed to create CString from _active_camera_id");
        let c_str = CString::new("")
            .expect("Failed to create CString from _active_camera_id");
        unsafe { 
            ACameraManager_openCamera(
                camera._camera_mgr, 
                c_str.as_ptr(),
                camera.get_device_listener(),

                // TODO...
                0 as *mut *mut ACameraDevice
                // &camera._cameras.get(camera._active_camera_id).unwrap().device
            );
        }

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

    /// Handle Camera DeviceStateChanges msg, notify device is disconnected
    /// simply close the camera
    pub fn on_device_state(&self, dev: *mut ACameraDevice) { 
        //TODO ...
        // std::string id(ACameraDevice_getId(dev));
        // LOGW("device %s is disconnected", id.c_str());
      
        // cameras_[id].available_ = false;
        // ACameraDevice_close(cameras_[id].device_);
        // cameras_.erase(id);
    }
    
    /// Handles Camera's deviceErrorChanges message, no action;
    /// mainly debugging purpose
    pub fn on_device_error(&self, dev: *mut ACameraDevice, err: i32) { 
        // TODO...
        // std::string id(ACameraDevice_getId(dev));

        // LOGI("CameraDevice %s is in error %#x", id.c_str(), err);
        // PrintCameraDeviceError(err);
      
        // CameraId& cam = cameras_[id];
      
        // switch (err) {
        //   case ERROR_CAMERA_IN_USE:
        //     cam.available_ = false;
        //     cam.owner_ = false;
        //     break;
        //   case ERROR_CAMERA_SERVICE:
        //   case ERROR_CAMERA_DEVICE:
        //   case ERROR_CAMERA_DISABLED:
        //   case ERROR_MAX_CAMERAS_IN_USE:
        //     cam.available_ = false;
        //     cam.owner_ = false;
        //     break;
        //   default:
        //     LOGI("Unknown Camera Device Error: %#x", err);
        // }
    }
    
    // pub fn on_session_state(&self, ses: &ACameraCaptureSession, state: CaptureSessionState) { }
    // pub fn on_capture_sequence_end(&self, session: ACameraCaptureSession, sequence_id: i32, frame_num: i64) { }
    // pub fn on_capture_failed(&self, session: &ACameraCaptureSession, request: &ACaptureRequest, failure: &ACameraCaptureFailure) { }

    pub fn start_preview(&self, _: bool) { }
    // pub fn take_photo(&self) -> bool { true }
    // pub fn get_exposure_range(&self, min: &i64, max: &i64, cur_val: &i64) -> bool { true }
    // pub fn get_sensitivity_range(&self, min: &i64, max: &i64, cur_val: &i64) -> bool { true }

    // pub fn update_camera_request_parameter(&self, code: i32, val: i64) { }

    fn get_device_listener(&self) -> *mut ACameraDevice_StateCallbacks {
        let on_disconnected: ACameraDevice_StateCallback
            = Some(on_device_state_changes);

        let on_error: ACameraDevice_ErrorStateCallback
            = Some(on_device_error_changes);

        let callbacks = ACameraDevice_StateCallbacks {
            // TODO...
            // see: https://stackoverflow.com/questions/33929079/rust-ffi-passing-trait-object-as-context-to-call-callbacks-on
            context: (0 as *mut c_void),
            onDisconnected: on_disconnected,
            onError: on_error
        };

        let static_ref: &'static mut ACameraDevice_StateCallbacks 
            = Box::leak(Box::new(callbacks));

        static_ref
    }
}
