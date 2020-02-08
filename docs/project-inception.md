# CartoonifyIt - Project Inception

## Goal

The goal of this project is to provide a real use case in which Washington Web Apps may gain
experience with low-level systems programming. This experience will be a valuable asset for
any project where resources are limited or real-time processing is involved.

## System Overview

The main component of this project is the `CartoonifyIt` Android application, a simple
Augmented Reality (AR) demo built with Rust and Android NDK. When launched, the application
presents a user with a real-time view of the camera, while applying on-the-fly color transforms
to appear as a cartoon to the user. Additionally, a user may be able to change the current visual
effect and save snapshots to the photo album. Possible future versions may provide the ability
to install 3rd-party plugins for additional visual effects.

The application is not intended as a revenue stream and will be released to the Play Store
under an open-source license, free and without adds. While intended as a demo, the end result
should be a well polished and user-friendly application that may be entertaining to users in most
target demographics.


## Initial Assumptions

The viability of this project is predicated on the following assumptions:

1. SHOULD be technically possible to implement this use case

    - The `ndk-samples/camera` [1] project demonstrates how one may access and manipulate live camera data using JNI to bridge our native library using Android NDK
    
    - A cartoon effect can be applied to a bitmap image using the OpenCV library. [2]

2. MAY be possible to apply these effects in real-time at a 12fps latency or better

3. MAY be able to learn rust-lang sufficiently enough to accomplish 1 and 2
    

[1] https://github.com/android/ndk-samples/tree/master/camera

[2] https://heartbeat.fritz.ai/image-effects-for-android-using-opencv-cartoon-effect-9d1c8c257b2a

## Roadmap

This roadmap is tentative and will be further developed in the `02-elaboration` phase. Major milestones are listed with their goals below.

### 01-inception

- Create a project inception document
- Establish requirements for MVP (1.0) release
- Successfully build "Hello, World" app locally and in CI pipeline

### 02-elaboration

- Produce detailed functional requirements for MVP release
- Implement a working BDD (functional) test suite
- Release an installable prototype that demonstrates basic camera functionality

### 03-construction

- Produce a 1.0 MVP release candidate of the application

### 04-transition

- Integrate release branding and certs
- Publish any end-user documentation, including developer guides, API docs, and applicable reports
- Produce an installable release build of the APK
- Publish Application to Play Store