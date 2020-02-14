# This Dockerfile creates a static build image for CI

FROM openjdk:8-jdk

# Just matched `app/build.gradle`
ENV ANDROID_COMPILE_SDK "28"
# Just matched `app/build.gradle`
ENV ANDROID_BUILD_TOOLS "28.0.3"
# Version from https://developer.android.com/studio/releases/sdk-tools
ENV ANDROID_SDK_TOOLS "26.1.1"

ENV ANDROID_NDK_REV "21.0.6113669"

ENV ANDROID_SDK_TOOLS_REV "4333796"

ENV ANDROID_CMAKE_REV "3.10.2.4988404"

ENV ANDROID_LLDB_REV "3.1"

ENV ANDROID_HOME /android-sdk-linux
ENV NDK_HOME "$ANDROID_HOME/ndk-bundle"
ENV PATH="${PATH}:/android-sdk-linux/platform-tools:${ANDROID_HOME}/tools/bin:${NDK_HOME}"

# install OS packages
RUN apt-get --quiet update --yes
RUN apt-get --quiet install --yes wget apt-utils tar unzip lib32stdc++6 lib32z1 build-essential
# We use this for xxd hex->binary
RUN apt-get --quiet install --yes vim-common

# Install Rust-Lang
ENV HOME /root
ENV CARGO_HOME $HOME/.cargo
RUN mkdir $CARGO_HOME
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="${PATH}:${CARGO_HOME}/bin"
RUN rustup --version; \
    cargo --version; \
    rustc --version

# Install Rust Build Targets
RUN rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# install Android SDK
RUN mkdir -p ${ANDROID_HOME} \
    && wget --quiet --output-document=${ANDROID_HOME}/android-sdk.zip https://dl.google.com/android/repository/sdk-tools-linux-${ANDROID_SDK_TOOLS_REV}.zip \
    && unzip -qq ${ANDROID_HOME}/android-sdk.zip -d ${ANDROID_HOME} \
    && rm ${ANDROID_HOME}/android-sdk.zip \
    && mkdir -p $HOME/.android \
    && echo 'count=0' > $HOME/.android/repositories.cfg

# Configure SDK packages

RUN yes | sdkmanager --licenses > /dev/null
RUN yes | sdkmanager --update > /dev/null
RUN yes | sdkmanager 'tools' > /dev/null
RUN yes | sdkmanager 'platform-tools' > /dev/null
RUN yes | sdkmanager 'build-tools;'$ANDROID_BUILD_TOOLS > /dev/null
RUN yes | sdkmanager 'platforms;android-'$ANDROID_COMPILE_SDK > /dev/null
RUN yes | sdkmanager 'platforms;android-28' > /dev/null
RUN yes | sdkmanager 'extras;android;m2repository' > /dev/null
RUN yes | sdkmanager 'extras;google;google_play_services' > /dev/null
RUN yes | sdkmanager 'extras;google;m2repository' > /dev/null
RUN yes | sdkmanager 'cmake;'$ANDROID_CMAKE_REV > /dev/null
RUN yes | sdkmanager 'lldb;'$ANDROID_LLDB_REV > /dev/null
RUN yes | sdkmanager 'ndk-bundle' > /dev/null
