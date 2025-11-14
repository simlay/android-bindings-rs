use android_activity::{
    input::{InputEvent, KeyAction, KeyEvent, KeyMapChar, MotionAction},
    InputStatus, MainEvent, PollEvent,
    WindowManagerFlags,
};
use android_bindings::{
    AndroidAppActivity,
    AndroidAppNativeActivity,
    AndroidContentContext, AndroidGraphicsColor,
    AndroidViewAutofillAutofillManager, AndroidViewViewGroupLayoutParams, AndroidViewWindow,
    AndroidWidgetEditText, AndroidWidgetLinearLayout, AndroidWidgetLinearLayoutLayoutParams,
    AndroidWidgetRelativeLayout,
    AndroidWidgetTextView, JavaLangCharSequence,
    AndroidWidgetButton,
    AndroidViewSurfaceView,
    //AndroidR,
    AndroidViewViewGroup,
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
    let config = app.config();
    println!("CONFIG : {config:#?}");

    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let _vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

    let context = AndroidContentContext::from(unsafe { JObject::from_raw(ctx.context().cast()) });

    // This works in java and android studio:
    // https://stackoverflow.com/a/39515370

    let activity =
        AndroidAppNativeActivity::from(unsafe { JObject::from_raw(app.activity_as_ptr().cast()) });
    let activity = activity.as_android_app_activity();

    // TODO: use this call  activity.run_on_ui_thread(...)

    let jstring = env.new_string("Text View from Rust!").expect("Failed to build string");
    let jchar_seq = JavaLangCharSequence::from(jstring);


    let text_view = AndroidWidgetTextView::new_1android_widget_text_view_landroid_content_context_2(
        env, context,
    );
    text_view.set_text_keep_state_ljava_lang_char_sequence_2(
        env,
        jchar_seq,
    );
    // Set white background for visibility
    text_view.as_android_view_view().set_background_color(env, 0xFFFFFFFF_u32 as i32);

    // Set black text color (visible on white background)
    text_view.set_text_color_i(env, 0xFF000000_u32 as i32);
    text_view.set_text_size_f(env, 48.);
    text_view.as_android_view_view().set_elevation(env, 100.);

    let window = activity.get_window(env);

    let content_view = activity.get_window(env).get_decor_view(env);
    let surface_view = AndroidViewSurfaceView::from(*content_view);
    //surface_view.set_z_order_on_top(env, false);
    //surface_view.set_z_order_media_overlay(env, true);
    let content_view = AndroidViewViewGroup::from(*content_view);
    //content_view.remove_all_views(env);
    println!("CHILD COUNT: {}", content_view.get_child_count(env));

    // Create layout parameters: WRAP_CONTENT (-2) for both width and height
    let layout_params = AndroidViewViewGroupLayoutParams::new_1android_view_view_group_024layout_params_ii(
        env,
        -2,  // WRAP_CONTENT
        -2,  // WRAP_CONTENT
    );

    // Add view with layout parameters
    content_view.add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        text_view.as_android_view_view(),
        layout_params,
    );
    println!("CHILD COUNT: {}", content_view.get_child_count(env));

    /*
    let button = AndroidWidgetButton::new_1android_widget_button_landroid_content_context_2(env, context);
    //button.as_android_widget_text_view().set_text_ljava_lang_char_sequence_2
    lin_layout.as_android_view_view_group().add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        button.as_android_widget_text_view().as_android_view_view(),
        lp_view
    );
    */

    /*
    let null_bundle = AndroidOsBundle::from(JObject::null());
    let null_persistable_bundle = AndroidOsPersistableBundle::from(JObject::null());
    activity.on_create(env, null_bundle, null_persistable_bundle);
    */

    /*
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as *mut _) };
    let activity_class = env.get_object_class(activity).expect("Failed to get activity class");
    let super_class = env.get_superclass(activity_class).expect("Failed to get super glass from activity");
    let on_create_method = env.get_method_id(
        super_class,
        "onCreate",
        "(Landroid/os/Bundle;)V"
    ).expect("Failed to get method id");;

    env.call_method_unchecked(
        activity,
        on_create_method,
        jaffi_support::jni::signature::ReturnType::Primitive(jaffi_support::jni::signature::Primitive::Void),
        &[JValue::Object(JObject::null()).into()],
    ).expect("Failed to call onCreate");
    */

    println!("Added the text editor view");

    Ok(())
}
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let args = std::env::args();

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
    println!("Android_main: {}", std::backtrace::Backtrace::force_capture());
}
