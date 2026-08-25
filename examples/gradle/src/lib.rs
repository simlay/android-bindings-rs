#[no_mangle]
pub extern "system" fn Java_com_example_GradleActivity_nativeInit(
    _env: *mut jni::sys::JNIEnv,
    _class: *mut std::ffi::c_void,
) {
    log::debug!("Rust library initialized successfully!");
}

#[no_mangle]
pub extern "system" fn Java_com_example_GradleActivity_showHelloWorld(
    _env: *mut jni::sys::JNIEnv,
    _class: *mut std::ffi::c_void,
) {
    log::debug!("showHelloWorld called!");
}
