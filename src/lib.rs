pub use crate::bindings::{
    //AndroidGraphicsRenderEffectClass
    //AndroidViewKeyEvent,
    //AndroidAnimationStateListAnimator,
    //AndroidWidgetEditText,
    //AndroidContentContext,
    //AndroidViewView,
    *,
};

mod bindings {
    #![allow(
        dead_code,
        clippy::unused_unit,
        clippy::needless_lifetimes,
        clippy::let_unit_value,
        clippy::let_and_return
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
    impl From<jaffi_support::jni::objects::JString<'_>> for JavaLangCharSequence<'_> {
        fn from(jstring: jaffi_support::jni::objects::JString<'_>) -> Self {
            JavaLangCharSequence::from(unsafe { JObject::from_raw(jstring.into_raw()) })
        }
    }
}

//mod generated_jaffi;
