# Introduction

The `CartoonifyIt` project is a demo Augmented Reality (AR) application built with Rust and the Android NDK.
The goal of this project is to gain experience with low-level systems programming in Rust, while providing
a useful and interesting domain to experiment in. The result of this demo will be published to the Play Store
under the MIT license, free to use and without adds.

See the [Project Inception Document](docs/project-inception.md) for more details on the project mission and timelines.

This project is a work in progress. The master branch is set to the [0.1.1](https://github.com/drkstr101/CartoonifyIt/releases/tag/v0.1.1) release which my serve as starting point for building your own android native activities in rust. When launched, you should be present you with a blank window and the application title.

Active development is currently being done in the [02-appengine](https://github.com/drkstr101/CartoonifyIt/tree/02-appengine) branch.


# Getting started

First thing is to follow the [Android tutorial](https://developer.android.com/training/basics/firstapp/) and
get Android Studio installed on your machine, so you can do development using
the Android IDE. Other IDE options are possible, but not directly described or
supported here. If you're using your own IDE, it should be fairly straightforward
to convert these instructions to use with your preferred toolchain.

The build environment will need to be configured for rust android development.
A getting started started guide may be found in the [rust-android-gradle](https://github.com/mozilla/rust-android-gradle/blob/master/README.md) project. This project has ffi bindings that rartget NDK `r20`, so make sure you have this version installed.

Once the build environment has the necessary tools installed, simply run the project in Android Studio, or build all targets with the `./gradlew build` command.

## What's in This Project

### Android Application (`app`)

The application source is based off the [Android NDKCamera](https://github.com/android/ndk-samples/tree/master/camera/basic) sample application, with the native component implemented in Rust.

### Native Rust Activity (`native_app`)

The rust native activity implements the main UI thread, and provides an injected `android_native_app_glue` based on the [android-rs-glue](https://github.com/rust-windowing/android-rs-glue) crate. The glue code has been forked so that it may be injected without depending on the cargo build task to package your apk. Instead we use the `rust-android-gradle` plugin provided by Mozilla to build and package the apk.

### Safe Rust Bindings for NDKCamera (`ndk_camera`)

This library provides a safe wrapper around the [Android NDK Camera API](https://developer.android.com/ndk/reference/group/camera)

### FFI Bindings for NDKCamera (`ndk_camera_sys`)

This library generates the FFI bindings for the native symbols provided by `android.so`. These bindings are based on `android-ndk-rs`.
Running the `generate_bindings.sh` script requires `bindgen` installed in your build environment.

### Dockerfile Environment (`Dockerfile`)

In the root there is a Dockerfile which defines a build environment which will be
used to ensure consistent and reliable builds of your Android application using
the correct Android SDK and other details you expect. Feel free to add any
build-time tools or whatever else you need here.

# References

- [Blog post: Android publishing with GitLab and fastlane](https://about.gitlab.com/2019/01/28/android-publishing-with-gitlab-and-fastlane/)
- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)