//mod bindings;
pub const ANDROID_R_ID_CONTENT: i32 = 16908290;
pub const ANDROID_R_COLOR_TRANSPARENT: i32 = 17170445;

use jni::objects::{JClass, JValue, JObject};
use jni::strings::JNIStr;
use jni::sys::jlong;
use jni::Env;
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
    println!("LOADING NATIVE CLASS");
    let class = load_native_runnable_class(env)?;
    println!("LOADED NATIVE CLASS");
    // This is probably wrong.
    let obj = env.new_object(class, jni_sig!((a: void) -> void), &[JValue::Long(ptr)])?;
    let runnable = unsafe { bindings::java::lang::Runnable::from_raw(env, *obj) };
    Ok(runnable)
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
        println!("FINDING THE CLASS IN NATIVE RUNNABLE");
        let byte_array = unsafe { jni::objects::JObject::from_raw(env, env.byte_array_from_slice(DEX_BYTES).unwrap().into_raw()) };

        let class_name = jni_str!("com/example/NativeRunnable");
        let class = env.find_class(class_name).expect("Failed to get NativeRunable class");

        let byte_buffer = env.call_static_method(
            env.find_class(jni_str!("java/nio/ByteBuffer")).expect("Failed to get ByteBuffer"),
            jni_str!("wrap"),
            jni_sig!(([byte]) -> java.nio.ByteBuffer),// "([B)Ljava/nio/ByteBuffer;",
            &[JValue::Object(&byte_array)],
        ).unwrap().l().unwrap();

        let dex_loader = env.new_object(
            env.find_class(jni_str!("dalvik/system/InMemoryDexClassLoader")).expect("Failed to get InMemoryDexClassLoader"),
            jni_sig!((java.nio.ByteBuffer, java.lang.ClassLoader) -> void), //"(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
            &[
            JValue::Object(&byte_buffer),
            JValue::Object(&jni::objects::JObject::null()),
            ],
        ).unwrap();
        // For now, we need to compile the NativeRunnable.java to a class file
        // and include it. Let's create a simple approach using the jni crate.
        let loaded = env.call_method(
            dex_loader,
            jni_str!("loadClass"),
            jni_sig!((java.lang.String) -> java.lang.Class), //"(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&JObject::from_raw(env, class.get_name(env).expect("Failed to get name").into_raw()))],
        ).unwrap().l().unwrap();
        let class = unsafe {jni::objects::JClass::from_raw(env, loaded.as_raw()) };
        println!("GOT CLASS IN NATIVE RUNNABLE");

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
            ).expect("Failed to register native methonds for NativeRunnable");
        }

        let global = env.new_global_ref(&class).expect("Failed to get global ref for native runnable");;
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
