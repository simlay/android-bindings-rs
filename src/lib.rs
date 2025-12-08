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
    /*
    */
}

//mod generated_jaffi;
