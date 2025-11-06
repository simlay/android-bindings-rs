use android_activity::{
    input::{InputEvent, KeyAction, KeyEvent, KeyMapChar, MotionAction},
    InputStatus, MainEvent, PollEvent,
};
use android_bindings::{
    AndroidAppActivity, AndroidContentContext, AndroidGraphicsColor,
    AndroidViewAutofillAutofillManager, AndroidViewViewGroupLayoutParams, AndroidViewWindow,
    AndroidWidgetEditText, AndroidWidgetLinearLayout, AndroidWidgetLinearLayoutLayoutParams,
    AndroidWidgetRelativeLayout,
    AndroidWidgetTextView, JavaLangCharSequence,
    AndroidWidgetButton,
    //AndroidR,
    AndroidWidgetTextViewBufferType,
};
use jaffi_support::jni::{
    objects::{JObject, JString, JValue},
    strings::{JNIStr, JNIString, JavaStr},
    sys::{jbyte, jchar},
    JNIEnv, JavaVM,
};
use log::info;
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder},
    platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid},
    raw_window_handle::{HasRawWindowHandle, RawWindowHandle},
};

pub struct App<'a> {
    android_app: AndroidApp,
    env: JNIEnv<'a>,
}
impl ApplicationHandler<()> for App<'_> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        println!("NEW EVENT: {cause:?}");
        if cause == winit::event::StartCause::Init {
            create_views(self.android_app.clone(), self.env).expect("Failed to create views");
        }
    }
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        //println!("RESUMED: {:?}", event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        //create_views(self.android_app.clone(), self.env).expect("Failed to create views");
        //let _ = ndk_context_jni_test(self.android_app.clone());
        println!("WINDOW EVENT: {:?}", event);
    }
}

/// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
fn create_views(
    app: AndroidApp,
    env: JNIEnv,
    //native_window: &ndk::native_window::NativeWindow,
) -> Result<(), Box<dyn std::error::Error>> {

    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let _vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

    let context = AndroidContentContext::from(unsafe { JObject::from_raw(ctx.context().cast()) });


    /*
    let text_view = AndroidWidgetTextView::new_1android_widget_text_view_landroid_content_context_2(
        env, context,
    );
    text_view
        .as_android_view_view()
        .set_background_color(env, 0x000000);
    */
    // This works in java and android studio:
    // https://stackoverflow.com/a/39515370
    let lin_layout =
        AndroidWidgetLinearLayout::new_1android_widget_linear_layout_landroid_content_context_2(
            env, context,
        );

    lin_layout.set_orientation(env, 1);
    /*
    lin_layout
        .as_android_view_view_group()
        .as_android_view_view()
        .set_padding(env, 16, 16, 16, 16);
    lin_layout
        .as_android_view_view_group()
        .add_view_landroid_view_view_2(env, text_view.as_android_view_view());
    */
    let lin_layout_param : AndroidViewViewGroupLayoutParams = AndroidViewViewGroupLayoutParams::new_1android_view_view_group_024layout_params_ii(
            env, -1, -1,
    );

    let activity =
        AndroidAppActivity::from(unsafe { JObject::from_raw(app.activity_as_ptr().cast()) });
    let window = activity.get_window(env);

    /*
    let decor_view = window.get_decor_view(env);
    let root_view = decor_view.find_view_by_id(env, 16908290);
    root_view.set_background_color(env, 0x00FF00);
    root_view.set_minimum_height(env, 1000);
    root_view.set_minimum_width(env, 1000);


    println!(
        "ROOT VIEW: {}, {}",
        root_view.get_height(env),
        root_view.get_width(env),
    );
    */
    /*
    window.set_content_view_i(
        env,
        16908290,
    );
    */
    window.set_content_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        lin_layout.as_android_view_view_group().as_android_view_view(),
        lin_layout_param
    );

    window.set_content_view_landroid_view_view_2(
        env,
        lin_layout.as_android_view_view_group().as_android_view_view(),
    );

    let lp_view : AndroidViewViewGroupLayoutParams = AndroidViewViewGroupLayoutParams::new_1android_view_view_group_024layout_params_ii(
            env, -2, -2,
    );

    let text_view =
        AndroidWidgetEditText::new_1android_widget_edit_text_landroid_content_context_2(
            env, context,
        );
    let jstring = env.new_string("foobar").expect("Failed to build string");
    let jchar_seq = JavaLangCharSequence::from(jstring);

    println!("STRING: {}", jchar_seq.to_string(env));
    text_view.as_android_widget_text_view().set_text_keep_state_ljava_lang_char_sequence_2(
        env,
        jchar_seq,
    );
    println!("STRING: {}", text_view.as_android_widget_text_view().get_text(env).to_string(env));
    /*
    */
    text_view.as_android_widget_text_view().as_android_view_view().set_layout_params(env, lp_view);
    lin_layout.as_android_view_view_group().add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        text_view.as_android_widget_text_view().as_android_view_view(),
        lp_view
    );

    /*
    let button = AndroidWidgetButton::new_1android_widget_button_landroid_content_context_2(env, context);
    //button.as_android_widget_text_view().set_text_ljava_lang_char_sequence_2
    lin_layout.as_android_view_view_group().add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        button.as_android_widget_text_view().as_android_view_view(),
        lp_view
    );
    */


    println!("Added the text editor view");

    Ok(())
}
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    info!("before hello world");
    println!("before hello world");
    let args = std::env::args();
    println!("ARGS:{args:?}");

    let mut event_loop: EventLoopBuilder<()> = EventLoop::with_user_event();
    event_loop.with_android_app(android_app.clone());
    let event_loop = event_loop.build().expect("Failed to build event loop");
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.expect("Failed to get vm");
    let env = vm
        .attach_current_thread_permanently()
        .expect("Failed to get env from vm");
    //ndk_context_jni_test(android_app.clone(), env);
    let mut winit_app = App { android_app, env };
    let _ = event_loop.run_app(&mut winit_app).expect("Fail to run app");

    info!("after hello world");
    println!("after hello world");
}
