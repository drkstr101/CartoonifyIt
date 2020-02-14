//! Raw FFI bindings needed by the native rust activity
//!
//! The bindings are pre-generated and the right one for the platform is selected at compile time.

// Bindgen lints
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::redundant_static_lifetimes)]

#[cfg(any(
    all(any(target_os = "android", test), target_arch = "arm"),
    feature = "rustdoc"
))]
include!("ffi_arm.rs");

#[cfg(all(any(target_os = "android", test), target_arch = "armv7"))]
include!("ffi_armv7.rs");

#[cfg(all(any(target_os = "android", test), target_arch = "aarch64"))]
include!("ffi_aarch64.rs");

#[cfg(all(any(target_os = "android", test), target_arch = "x86"))]
include!("ffi_i686.rs");

#[cfg(all(any(target_os = "android", test), target_arch = "x86_64"))]
include!("ffi_x86_64.rs");