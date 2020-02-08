fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=android_glue/android_native_app_glue.c");
    println!("cargo:rustc-link-lib=static=native_app_glue");
    cc::Build::new()
    .file("android_glue/android_native_app_glue.c")
    .compile("native_app_glue");
}
