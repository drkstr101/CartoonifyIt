//! Bindings for `ACameraCaptureSession`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera.html#acameracapturesession)

use std::ptr::NonNull;

/// An `ACameraCaptureSession *`
#[derive(Debug)]
pub struct CameraCaptureSession {
    ptr: NonNull<ffi::ACameraCaptureSession>,
}

impl CameraCaptureSession {
    /// Create a `CameraCaptureSession` from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACameraCaptureSession`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACameraCaptureSession>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACameraCaptureSession`
    pub fn ptr(&self) -> NonNull<ffi::ACameraCaptureSession> {
        self.ptr
    }
}