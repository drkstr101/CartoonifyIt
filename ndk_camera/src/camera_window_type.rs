//! Bindings for `ACameraWindowType`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera#acamerawindowtype)

use std::ptr::NonNull;

/// An `ACameraWindowType *`
#[derive(Debug)]
pub struct CameraWindowType {
    ptr: NonNull<ffi::ACameraWindowType>,
}

impl CameraWindowType {
    /// Create a `CameraWindowType` from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACameraWindowType`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACameraWindowType>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACameraWindowType`
    pub fn ptr(&self) -> NonNull<ffi::ACameraWindowType> {
        self.ptr
    }
}
