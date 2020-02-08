# Introduction

The `CartoonifyIt` project is a demo Augmented Reality (AR) application built with Rust and the Android NDK.
The goal of this project is to gain experience with low-level systems programming in Rust, while providing
a useful and interesting domain to experiment in. The result of this demo will be published to the Play Store
under the MIT license, free to use and without adds.

See the [Project Inception Document](docs/project-inception.md) for more details on the project mission and timelines.

This project is a work in progress. The master branch is set to the `0.1.1` release which my serve as starting point for building your own android native activities in rust. 

This is a "Hello, World!" implementation that when launched will present you with a blank window and the application title.


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

The rust native activity implements the main UI thread, and provides an injected `android_native_app_glue` based on the [android-rs-glue](https://github.com/rust-windowing/android-rs-glue) crate. The glue code has been forked so that it may be injected without depending on the cargo build task to package your apk. Instead we use the `rust-android-gradle` plugin provided by Mozilla to build and package the apk.


### Fastlane (`fastlane`)

It also has fastlane setup per the [blog post](https://about.gitlab.com/2019/01/28/android-publishing-with-gitlab-and-fastlane/) on
getting GitLab CI set up with fastlane. Note that you may want to update your
fastlane bundle to the latest version; if a newer version is available, the pipeline
job output will tell you.

### Dockerfile Environment (`Dockerfile`)

In the root there is a Dockerfile which defines a build environment which will be
used to ensure consistent and reliable builds of your Android application using
the correct Android SDK and other details you expect. Feel free to add any
build-time tools or whatever else you need here.

### GitLab CI (`.gitlab-ci.yml`)

The sample project also contains a basic `.gitlab-ci.yml` which will successfully 
build the Android application.

Note that for publishing to the test channels or production, you'll need to set
up your secret API key. The stub code is here for that, but please see our
[blog post](https://about.gitlab.com/2019/01/28/android-publishing-with-gitlab-and-fastlane/) for
details on how to set this up completely. In the meantime, publishing steps will fail.

The build script also handles automatic versioning by relying on the CI pipeline
ID to generate a unique, ever increasing number. If you have a different versioning
scheme you may want to change this.

```yaml
    - "export VERSION_CODE=$(($CI_PIPELINE_IID)) && echo $VERSION_CODE"
    - "export VERSION_SHA=`echo ${CI_COMMIT_SHA:0:8}` && echo $VERSION_SHA"
```


# References

- [Blog post: Android publishing with GitLab and fastlane](https://about.gitlab.com/2019/01/28/android-publishing-with-gitlab-and-fastlane/)
- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)