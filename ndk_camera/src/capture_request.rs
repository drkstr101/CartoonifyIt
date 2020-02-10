//! Bindings for `ACaptureRequest`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera#acapturerequest)

use std::ptr::NonNull;

/// An `ACaptureRequest *`
#[derive(Debug)]
pub struct CaptureRequest {
    ptr: NonNull<ffi::ACaptureRequest>,
}

impl CaptureRequest {
    /// Create a `CaptureRequest` from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACaptureRequest`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACaptureRequest>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACaptureRequest`
    pub fn ptr(&self) -> NonNull<ffi::ACaptureRequest> {
        self.ptr
    }
}