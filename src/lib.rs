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
    //let class = env.find_class("com/example/NativeRunnable")?;
    let class = load_native_runnable_class(env);
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
use std::sync::OnceLock;

static NATIVE_RUNNABLE_CLASS: OnceLock<jni::objects::GlobalRef> = OnceLock::new();

pub fn init(env: JNIEnv) {
    let class = load_native_runnable_class(env);
}
static DEX_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));

pub fn load_native_runnable_class (
    env: JNIEnv,
) -> jni::objects::JClass {
    if let Some(global) = NATIVE_RUNNABLE_CLASS.get() {
        JClass::from(global.as_obj())
    } else {
        let byte_array = unsafe { JObject::from_raw(env.byte_array_from_slice(DEX_BYTES).unwrap()) };
        let byte_buffer = env.call_static_method(
            "java/nio/ByteBuffer",
            "wrap",
            "([B)Ljava/nio/ByteBuffer;",
            &[byte_array.into()],
        ).unwrap().l().unwrap();

        let dex_loader = env.new_object(
            "dalvik/system/InMemoryDexClassLoader",
            "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
            &[
            byte_buffer.into(),
            jni::objects::JObject::null().into(),
            ],
        ).unwrap();

        let class_name = env.new_string("com.example.NativeRunnable").unwrap();
        let loaded = env.call_method(
            dex_loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[class_name.into()],
        ).unwrap().l().unwrap();

        let class = jni::objects::JClass::from(loaded);
        // Register native methods explicitly so JNI can find them
        env.register_native_methods(
            class,
            &[
            jni::NativeMethod {
                name: "nativeRun".into(),
                sig: "(J)V".into(),
                fn_ptr: Java_com_example_NativeRunnable_nativeRun as *mut std::ffi::c_void,
            },
            jni::NativeMethod {
                name: "nativeDrop".into(),
                sig: "(J)V".into(),
                fn_ptr: Java_com_example_NativeRunnable_nativeDrop as *mut std::ffi::c_void,
            },
            ],
        ).expect("Failed to register native methods");
        let global = env.new_global_ref(class).unwrap();
        NATIVE_RUNNABLE_CLASS.set(global).ok();
        class
    }
}
