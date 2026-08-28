//mod bindings;
pub const ANDROID_R_ID_CONTENT: i32 = 16908290;
pub const ANDROID_R_COLOR_TRANSPARENT: i32 = 17170445;

use jni::objects::{JClass, JValue};
use jni::strings::JNIStr;
use jni::sys::jlong;
use jni::{Env, EnvUnowned};
use std::panic::catch_unwind;
use std::sync::OnceLock;
use jni_macros::{
    jni_str,
    jni_sig,
};

type RunnableClosure = Box<dyn FnMut() + Send + 'static>;

/// Creates a Java Runnable from a Rust closure
pub fn create_runnable<'local, F>(
    env: &mut Env<'local>,
    closure: F,
) -> jni::errors::Result<bindings::java::lang::Runnable<'local>>
where
    F: FnMut() + Send + 'static,
{
    // Box the closure and convert to raw pointer
    let boxed: RunnableClosure = Box::new(closure);
    let ptr = Box::into_raw(Box::new(boxed)) as jlong;

    // Find NativeRunnable class and create instance
    log::info!("LOADING NATIVE CLASS");
    let class = load_native_runnable_class(env)?;
    log::info!("LOADED NATIVE CLASS");
    let obj = env.new_object(class, jni_sig!((jlong) -> void), &[JValue::Long(ptr)]).expect("Failed to create new object for native runnable");
    log::info!("Created new jobject for runnable");
    let runnable = unsafe { bindings::java::lang::Runnable::from_raw(env, *obj) };
    Ok(runnable)
}

/// Native method called when Runnable.run() is invoked
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_NativeRunnable_nativeRun(
    _env: EnvUnowned,
    _class: jni::sys::jclass,
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
    _env: EnvUnowned,
    _class: jni::sys::jclass,
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
static DEX_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
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
        log::info!("FINDING THE CLASS IN NATIVE RUNNABLE");
        let byte_array_raw = env.byte_array_from_slice(DEX_BYTES).unwrap().into_raw();
        let byte_array = unsafe { jni::objects::JObject::from_raw(env, byte_array_raw) };

        let byte_buffer_class = env.find_class(jni_str!("java/nio/ByteBuffer")).expect("Failed to get ByteBuffer");
        let byte_buffer = env.call_static_method(
            byte_buffer_class,
            jni_str!("wrap"),
            jni_sig!(([byte]) -> java.nio.ByteBuffer),// "([B)Ljava/nio/ByteBuffer;",
            &[JValue::Object(&byte_array)],
        ).unwrap().l().unwrap();

        let class_loader_class = env.find_class(jni_str!("dalvik/system/InMemoryDexClassLoader")).expect("Failed to get InMemoryDexClassLoader");
        let dex_loader = env.new_object(
            class_loader_class,
            jni_sig!((java.nio.ByteBuffer, java.lang.ClassLoader) -> void), //"(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
            &[
            JValue::Object(&byte_buffer),
            JValue::Object(&jni::objects::JObject::null()),
            ],
        ).unwrap();
        // For now, we need to compile the NativeRunnable.java to a class file
        // and include it. Let's create a simple approach using the jni crate.
        let class_name_jstring = env.new_string("com/example/NativeRunnable").expect("Failed to create class name string");
        let loaded = env.call_method(
            dex_loader,
            jni_str!("loadClass"),
            jni_sig!((java.lang.String) -> java.lang.Class), //"(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&class_name_jstring)],
        ).unwrap().l().unwrap();
        let class = unsafe { jni::objects::JClass::from_raw(env, loaded.as_raw()) };
        log::info!("GOT CLASS IN NATIVE RUNNABLE");

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
                    JNIStr::from_cstr(c"nativeDrop").expect("Failed to get jnistr"),
                    JNIStr::from_cstr(c"(J)V").expect("Failed to get JNIStr"),
                    Java_com_example_NativeRunnable_nativeDrop as *mut std::ffi::c_void,
                ),
                ],
            ).expect("Failed to register native methonds for NativeRunnable");
        }

        let global = env.new_global_ref(&class).expect("Failed to get global ref for native runnable");
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
