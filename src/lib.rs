pub use crate::bindings::{
    *,
};
const ANDROID_R_ID_CONTENT: i32 = 16908290;
const ANDROID_R_COLOR_TRANSPARENT: i32 = 17170445;

mod bindings {
    #![allow(
        dead_code,
        clippy::unused_unit,
        clippy::needless_lifetimes,
        clippy::let_unit_value,
        clippy::let_and_return,
        clippy::too_many_arguments,
        mismatched_lifetime_syntaxes,
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
    impl From<jaffi_support::jni::objects::JString<'_>> for java::lang::CharSequence<'_> {
        fn from(jstring: jaffi_support::jni::objects::JString<'_>) -> Self {
            Self::from(unsafe { JObject::from_raw(jstring.into_raw()) })
        }
    }
}
use jni::sys::jlong;
use std::panic::catch_unwind;

type RunnableClosure = Box<dyn FnMut() + Send + 'static>;
// Type alias for boxed closure

/// Creates a Java Runnable from a Rust closure
pub fn create_runnable<'local, F>(
    env: JNIEnv<'local>,
    closure: F,
) -> jni::errors::Result<java::lang::Runnable<'local>>
where
    F: FnMut() + Send + 'static,
{
    // Box the closure and convert to raw pointer
    let boxed: RunnableClosure = Box::new(closure);
    let ptr = Box::into_raw(Box::new(boxed)) as jlong;

    // Find NativeRunnable class and create instance
    let class = env.find_class("com/example/NativeRunnable")?;
    let obj = env.new_object(class, "(J)V", &[JValue::Long(ptr)])?;

    Ok(java::lang::Runnable::from(obj))
}

/// Native method called when Runnable.run() is invoked
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_NativeRunnable_nativeRun(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let _ = catch_unwind(|| {
        if ptr != 0 {
            let closure = unsafe { &mut *(ptr as *mut RunnableClosure) };
            closure();
        }
    });
}

/// Native method to drop/free the closure when no longer needed
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_NativeRunnable_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RunnableClosure);
            // Box is dropped here, freeing the closure
        }
    }
}
