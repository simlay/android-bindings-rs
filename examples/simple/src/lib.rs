/*
use android_activity::{
    input::{InputEvent, KeyAction, KeyEvent, KeyMapChar, MotionAction},
    InputStatus, MainEvent, PollEvent,
    WindowManagerFlags,
};
*/
/*
use android_bindings::{
    //AndroidAppActivity,
    AndroidAppNativeActivity,
    AndroidContentContext, AndroidGraphicsColor,
    AndroidViewAutofillAutofillManager, AndroidViewViewGroupLayoutParams, AndroidViewWindow,
    AndroidWidgetEditText, AndroidWidgetLinearLayout, AndroidWidgetLinearLayoutLayoutParams,
    //AndroidWidgetRelativeLayout,
    AndroidWidgetFrameLayout,
    AndroidWidgetTextView, JavaLangCharSequence,
    //AndroidWidgetButton,
    AndroidViewSurfaceView,
    //AndroidR,
    AndroidViewViewGroup,

    JavaLangRunnable,
};
*/
use jaffi_support::jni::{
    objects::JObject,
    //objects::{JObject, JString, JValue},
    //strings::{JNIStr, JNIString, JavaStr},
    //sys::{jbyte, jchar},
    JNIEnv, JavaVM,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder},
    platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid},
    //raw_window_handle::{HasRawWindowHandle, RawWindowHandle},
};

pub struct App<'a> {
    android_app: AndroidApp,
    env: JNIEnv<'a>,
}
impl ApplicationHandler<()> for App<'_> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        log::debug!("NEW EVENT: {cause:?}");
        if cause == winit::event::StartCause::Init {
            //create_views(self.android_app.clone(), self.env).expect("Failed to create views");
        }
    }
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        //log::debug!("RESUMED: {:?}", event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::debug!("WINDOW EVENT: {:?}", event);
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                //create_views(self.android_app.clone(), self.env).expect("Failed to create views");
            },
            winit::event::WindowEvent::Focused(true) => {
                //create_views(self.android_app.clone(), self.env).expect("Failed to create views");
            },
            _other => {

            },
            winit::event::WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            winit::event::WindowEvent::Resized(physical_size) => todo!(),
            winit::event::WindowEvent::Moved(physical_position) => todo!(),
            winit::event::WindowEvent::CloseRequested => todo!(),
            winit::event::WindowEvent::Destroyed => todo!(),
            winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            winit::event::WindowEvent::Focused(_) => todo!(),
            winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => todo!(),
            winit::event::WindowEvent::ModifiersChanged(modifiers) => todo!(),
            winit::event::WindowEvent::Ime(ime) => todo!(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => todo!(),
            winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
            winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
            winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::MouseInput { device_id, state, button } => todo!(),
            winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::DoubleTapGesture { device_id } => todo!(),
            winit::event::WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            winit::event::WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            winit::event::WindowEvent::Touch(touch) => todo!(),
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            winit::event::WindowEvent::ThemeChanged(theme) => todo!(),
            winit::event::WindowEvent::Occluded(_) => todo!(),
        }
        //create_views(self.android_app.clone(), self.env).expect("Failed to create views");
        //let _ = ndk_context_jni_test(self.android_app.clone());
    }
}

/// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
fn create_views(
    app: AndroidApp,
    env: JNIEnv,
    //native_window: &ndk::native_window::NativeWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = app.config();
    log::debug!("CONFIG : {config:#?}");

    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let _vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

    let ctx = android_bindings::android::content::Context::from(unsafe { JObject::from_raw(ctx.context().cast()) });
    //let window = app.native_window().expect("Failed to get window");
    //let (height, width) = (window.height(), window.width());
    //log::debug!("WINDOW HEIGHT: {height}, width: {width}");

    // This works in java and android studio:
    // https://stackoverflow.com/a/39515370

    let activity = android_bindings::android::app::NativeActivity::from(unsafe { JObject::from_raw(app.activity_as_ptr().cast()) });
    let activity = activity.as_activity();

    // TODO: use this call
    // activity.run_on_ui_thread(...)

    let jchar_seq = android_bindings::java::lang::CharSequence::from(env.new_string("Text View from Rust!")?);

    let text_view = android_bindings::android::widget::TextView::new_1android_widget_text_view_landroid_content_context_2(
        env, ctx,
    );

    text_view.set_text_keep_state_ljava_lang_char_sequence_2(
        env,
        jchar_seq,
    );

    // Set white background for visibility
    text_view.as_view().set_background_color(env, 0xFFFFFFFF_u32 as i32);

    // Set black text color (visible on white background)
    text_view.set_text_color_i(env, 0xFF000000_u32 as i32);
    text_view.set_text_size_f(env, 48.);
    text_view.as_view().set_elevation(env, 100.);

    let window = activity.get_window(env);

    let frame_layout = android_bindings::android::widget::FrameLayout::new_1android_widget_frame_layout_landroid_content_context_2(env, ctx);

    frame_layout.as_view_group().add_view_landroid_view_view_2(env, text_view.as_view());
    window.set_content_view_landroid_view_view_2(env, frame_layout.as_view_group().as_view());
    frame_layout.as_view_group().as_view().invalidate(env);
    frame_layout.as_view_group().as_view().measure(env, 1000, 1000);

    /*
    let decor_view = window.get_decor_view(env);
    let content_view = AndroidViewViewGroup::from(*decor_view);
    log::debug!("CHILD COUNT: {}", content_view.get_child_count(env));
    let content_view = AndroidViewViewGroup::from(
        *content_view.get_child_at(env, 0)
    );

    log::debug!("CHILD COUNT: {}", content_view.get_child_count(env));
    let content_view = AndroidViewViewGroup::from(
        *content_view.get_child_at(env, 1)
    );

    //content_view.remove_all_views(env);
    log::debug!("CHILD COUNT: {}", content_view.get_child_count(env));

    // Create layout parameters: WRAP_CONTENT (-2) for both width and height
    let layout_params = AndroidViewViewGroupLayoutParams::new_1android_view_view_group_024layout_params_ii(
        env,
        -1,  // MATCH_PARENT
        -1,  // MATCH_PARENT
    );

    content_view.add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        lin_layout.as_android_view_view_group().as_android_view_view(),
        layout_params,
    );
    lin_layout.as_android_view_view_group().add_view_landroid_view_view_2(env, text_view.as_android_view_view());
    */

    /*
    // Add view with layout parameters
    content_view.add_view_landroid_view_view_2landroid_view_view_group_024layout_params_2(
        env,
        text_view.as_android_view_view(),
        layout_params,
    );
    log::debug!("CHILD COUNT: {}", content_view.get_child_count(env));
    */

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

    log::debug!("Added the text editor view");

    Ok(())
}
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
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
    log::debug!("Android_main: {}", std::backtrace::Backtrace::force_capture());
}
