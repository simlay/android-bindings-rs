//mod bindings;
pub const ANDROID_R_ID_CONTENT: i32 = 16908290;
pub const ANDROID_R_COLOR_TRANSPARENT: i32 = 17170445;

use jni::objects::{JClass, JValue};
use jni::strings::JNIStr;
use jni::sys::jlong;
use jni::Env;
use std::panic::catch_unwind;
use std::sync::OnceLock;
use jni_macros::jni_sig;

type RunnableClosure = Box<dyn FnMut() + Send + 'static>;

/// Creates a Java Runnable from a Rust closure
pub fn create_runnable<'local, F>(
    mut env: Env<'local>,
    closure: F,
) -> jni::errors::Result<jni::objects::JObject<'local>>
where
    F: FnMut() + Send + 'static,
{
    // Box the closure and convert to raw pointer
    let boxed: RunnableClosure = Box::new(closure);
    let ptr = Box::into_raw(Box::new(boxed)) as jlong;

    // Find NativeRunnable class and create instance
    let class = load_native_runnable_class(&mut env)?;
    // This is probably wrong.
    let obj = env.new_object(class, jni_sig!((a: void) -> void), &[JValue::Long(ptr)])?;

    Ok(obj)
}

/// Native method called when Runnable.run() is invoked
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_NativeRunnable_nativeRun(
    _env: Env,
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
    _env: Env,
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

static NATIVE_RUNNABLE_CLASS: OnceLock<jni::objects::Global<JClass>> = OnceLock::new();

pub fn load_native_runnable_class<'local>(
    env: &mut Env<'local>,
) -> jni::errors::Result<JClass<'local>> {
    if let Some(global) = NATIVE_RUNNABLE_CLASS.get() {
        // Convert GlobalRef to JClass
        // GlobalRef::as_obj() returns &JObject<'static>, which can be transmuted to &JClass<'static>
        let obj: &jni::objects::JObject<'static> = global.as_ref();
        let class_ref: &jni::objects::JClass<'static> = unsafe { std::mem::transmute(obj) };
        // Create a new JClass<'local> from the raw pointer
        let raw_ptr = class_ref.as_raw();
        Ok(unsafe { JClass::from_raw(env, raw_ptr) })
    } else {
        // For now, we need to compile the NativeRunnable.java to a class file
        // and include it. Let's create a simple approach using the jni crate.

        // Find or load the NativeRunnable class
        let class_name = JNIStr::from_cstr(c"com/example/NativeRunnable").expect("Failed to get classname");
        let class = env.find_class(class_name)?;

        unsafe {
            // Register native methods explicitly so JNI can find them
            env.register_native_methods(
                &class,
                &[
                jni::NativeMethod::from_raw_parts(
                    JNIStr::from_cstr(c"nativeRun").expect("Failed to get jnistr"),
                    JNIStr::from_cstr(c"(J)V").expect("Failed to get JNIStr"),
                    Java_com_example_NativeRunnable_nativeRun as *mut std::ffi::c_void,
                ),
                jni::NativeMethod::from_raw_parts(
                    JNIStr::from_cstr(c"nativeRun").expect("Failed to get jnistr"),
                    JNIStr::from_cstr(c"(J)V").expect("Failed to get JNIStr"),
                    Java_com_example_NativeRunnable_nativeDrop as *mut std::ffi::c_void,
                ),
                ],
            )?;
        }

        let global = env.new_global_ref(&class)?;
        NATIVE_RUNNABLE_CLASS.set(global).ok();
        Ok(class)
    }
}

// Include the generated bindings from build.rs
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}
/*
*/
