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
}
//mod generated_jaffi;
