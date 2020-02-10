//! Bindings for `ACameraDevice`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera.html#acameradevice)

use std::ptr::NonNull;

/// An `ACameraDevice *`
#[derive(Debug)]
pub struct CameraDevice {
    ptr: NonNull<ffi::ACameraDevice>,
}

impl CameraDevice {
    /// Create a `CameraDevice` from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACameraDevice`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACameraDevice>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACameraDevice`
    pub fn ptr(&self) -> NonNull<ffi::ACameraDevice> {
        self.ptr
    }
}