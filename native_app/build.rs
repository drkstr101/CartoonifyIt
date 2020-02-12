/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};

fn llvm_arch() -> &'static str {
    let target = env::var("TARGET").unwrap();
    match target.as_ref() {
        "aarch64-linux-android" => "aarch64",
        "armv7-linux-androideabi" => "armv7a",
        "i686-linux-android" => "i686",
        "x86_64-linux-android" => "x86_64",
        _ => panic!("Unknown android cross-compile target: {}", target)
    }
}

fn llvm_host() -> &'static str {
    let host = env::var("HOST").unwrap();
    match host.as_ref() {
        "i686-unknown-linux-gnu" => "linux-x86",
        "x86_64-apple-darwin" => "darwin-x86_64",
        "x86_64-unknown-linux-gnu" => "linux-x86_64",
        _ => panic!("Unknown support android cross-compile host: {}", host)
    }
}

/// Returns the path to the LLVM toolchain provided by the NDK
fn llvm_toolchain_root(ndk_path: &Path) -> PathBuf {
    ndk_path
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(llvm_host())
}

fn llvm_clang(toolchain_root: &Path) -> PathBuf {
    let arch = llvm_arch();
    let name = match arch.as_ref() {
        "aarch64" => "aarch64-linux-android28-clang",
        "armv7a" => "armv7a-linux-androideabi28-clang",
        "i686" => "i686-linux-android28-clang",
        "x86_64" => "x86_64-linux-android28-clang",
        _ => panic!("Unknown LLVM arch: {}", arch)
    };

    toolchain_root
        .join("bin")
        .join(name)
}

fn llvm_ar(toolchain_root: &Path) -> PathBuf {
    let arch = llvm_arch();
    let name = match arch.as_ref() {
        "aarch64" => "aarch64-linux-android-ar",
        "armv7a" => "arm-linux-androideabi-ar",
        "i686" => "i686-linux-android-ar",
        "x86_64" => "x86_64-linux-android-ar",
        _ => panic!("Unknown LLVM arch: {}", arch)
    };

    toolchain_root
        .join("bin")
        .join(name)
}
 
fn main() {

    let out_dir = env::var("OUT_DIR").ok()
        .expect("Cargo should have set the OUT_DIR environment variable");
    
    println!("cargo:rustc-link-lib=static=android_native_app_glue");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=log");
    println!("cargo:rustc-link-lib=android");

    // Get the NDK path from NDK_HOME env.
    let ndk_path = env::var_os("NDK_HOME")
        .expect("Please set the NDK_HOME environment variable");
    let ndk_path = Path::new(&ndk_path);
    let toolchain_root = llvm_toolchain_root(&ndk_path);

    println!("toolchain path is: {}", toolchain_root.to_str().unwrap());
    assert!(toolchain_root.exists());

    
    let c_file = ndk_path
    .join("sources")
        .join("android")
        .join("native_app_glue")
        .join("android_native_app_glue.c");

    
    let clang_cmd = llvm_clang(&toolchain_root);
    println!("Using Clang: {}", clang_cmd.to_str().unwrap());
    assert!(clang_cmd.exists());

    let sysroot = toolchain_root.join("sysroot");
    println!("Using --sysroot {}", sysroot.to_str().unwrap());
    assert!(sysroot.exists());

    let out_dir = Path::new(&out_dir);

    // compiling android_native_app_glue.c
    if Command::new(clang_cmd)
        .arg(c_file)
        .arg("-c")
        .arg("-o").arg(out_dir.join("android_native_app_glue.o"))
        .arg("--sysroot").arg(sysroot)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().unwrap().code().unwrap() != 0
    {
        println!("Error while executing clang");
        process::exit(1)
    }

    let ar_cmd = llvm_ar(&toolchain_root);
    println!("Using Clang: {}", ar_cmd.to_str().unwrap());
    assert!(ar_cmd.exists());
        
    // compiling libandroid_native_app_glue.a
    if Command::new(llvm_ar(&toolchain_root))
        .arg("rcs")
        .arg(out_dir.join("libandroid_native_app_glue.a"))
        .arg(out_dir.join("android_native_app_glue.o"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().unwrap().code().unwrap() != 0
    {
        println!("Error while executing ar");
        process::exit(1)
    }
      
}