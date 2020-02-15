# Introduction

**This project is a work in progress**. Please see active branches for ongoing development.

The `CartoonifyIt` project is a demo Augmented Reality (AR) application built with Rust and the Android NDK.
The goal of this project is to gain experience with low-level systems programming in Rust, while providing
a useful and interesting domain to experiment in. The result of this demo will be published to the Play Store
under the MIT license, free to use and without adds.

See the [Project Inception Document](docs/project-inception.md) for more details on the project mission and timelines.


# Getting started

First thing is to follow the [Android tutorial](https://developer.android.com/training/basics/firstapp/) and
get Android Studio installed on your machine, so you can do development using
the Android IDE. Other IDE options are possible, but not directly described or
supported here. If you're using your own IDE, it should be fairly straightforward
to convert these instructions to use with your preferred toolchain.

The build environment will need to be configured for rust android development.
A getting started started guide may be found in the [rust-android-gradle](https://github.com/mozilla/rust-android-gradle/blob/master/README.md) project.

Once the build environment has the necessary tools installed, simply run the project in Android Studio, or build all targets with the `./gradlew build` command.

## What's in This Project

### Android Application (`app`)

The state of this project is as if you followed the first few steps in the linked
[Android tutorial](https://developer.android.com/training/basics/firstapp/) and
have created your project. The application source is based off the [Android NDKCamera](https://github.com/android/ndk-samples/tree/master/camera/basic) sample application, with the native component implemented in Rust.

### Rust Native Activity (`native_app`)

This is a Cargo crate which contains the rust implementation of the native activity ported from C++. We use the `rust-android-gradle` plugin provided by Mozilla to build and package the native code into the apk.

### Rust FFI Bindings for Native Activity (`native_app_sys`)

This is a Cargo crate which holds the generated ffi bindings used by the native activity. See the `generate-bindings.sh` script to see how these bindings are generated. Use `cargo install bindgen` before running the script.

### Dockerfile Environment (`Dockerfile`)

In the root there is a Dockerfile which defines a build environment which will be
used to ensure consistent and reliable builds of your Android application using
the correct Android SDK and other details you expect. Feel free to add any
build-time tools or whatever else you need here.

# References

- [Blog post: Android publishing with GitLab and fastlane](https://about.gitlab.com/2019/01/28/android-publishing-with-gitlab-and-fastlane/)
- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)