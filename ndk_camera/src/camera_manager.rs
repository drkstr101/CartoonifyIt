//! Bindings for `ACameraManager`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/group/camera#acameramanager)

use std::ptr::NonNull;

/// An `ACameraManager *`
#[derive(Debug)]
pub struct CameraManager {
    ptr: NonNull<ffi::ACameraManager>,
}

impl Drop for CameraManager {
    fn drop(&mut self) {
        unsafe { ffi::ACameraManager_delete(self.ptr.as_ptr()) }
    }
}

impl CameraManager {
    /// Create a `CameraManager` from a pointer
    /// 
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ACameraManager`.
    ///
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ACameraManager>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ACameraManager`
    pub fn ptr(&self) -> NonNull<ffi::ACameraManager> {
        self.ptr
    }
    
    /// Create a new `CameraManager`, with none of the values set.
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ffi::ACameraManager_create()).unwrap(),
            }
        }
    }
}
