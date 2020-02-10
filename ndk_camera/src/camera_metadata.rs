//! Bindings for `ACameraMetadata`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera.html#acamerametadata)

use std::ptr::NonNull;

/// An `ACameraMetadata *`
#[derive(Debug)]
pub struct CameraMetadata {
    ptr: NonNull<ffi::ACameraMetadata>,
}

impl CameraMetadata {
    /// Create a `CameraMetadata` from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACameraMetadata`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACameraMetadata>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACameraMetadata`
    pub fn ptr(&self) -> NonNull<ffi::ACameraMetadata> {
        self.ptr
    }
}