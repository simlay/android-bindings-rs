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
    JNIEnv,
    JavaVM,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder},
    platform::android::{
        activity::AndroidApp, EventLoopBuilderExtAndroid,
    },
};
use ndk::hardware_buffer_format::HardwareBufferFormat;

pub struct App<'a> {
    android_app: AndroidApp,
    env: JNIEnv<'a>,
}
impl<'a> App<'a> {
    fn create_views(&self) {
        //self.fix_surface_view();
        let android_app = self.android_app.clone();
        let vm = self.env.get_java_vm().expect("Failed to get JavaVM");

        let runnable = android_bindings::create_runnable(self.env, move || {
            println!("THIS IS RAN ON THE UI THREAD");
            let env = vm.attach_current_thread().expect("Failed to attach thread");
            Self::print_tree(android_app.clone(), *env).expect("Failed to create views");
            //Self::claude_suggestion_1(android_app.clone(), *env).expect("Failed to create views");
            //Self::create_views_on_ui_thread(android_app.clone(), *env).expect("Failed to create views");
        }).expect("Failed to build runnable");
        let activity = android_bindings::android::app::NativeActivity::from(unsafe {
            JObject::from_raw(self.android_app.activity_as_ptr().cast())
        });
        let activity = activity.as_activity();
        activity.run_on_ui_thread(self.env, runnable);
    }
    pub fn fix_surface_view(&self) {
        if let Some(native_window) = self.android_app.native_window() {
            unsafe {
                ndk_sys::ANativeWindow_release(native_window.ptr().as_ptr());
            }
            println!("release_window: CALLED ANativeWindow_release");
        } else {
            println!("release_window: THERE IS NO NATIVE WINDOW");
        }
    }
    fn print_tree(
        app: AndroidApp,
        env: JNIEnv,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let activity = android_bindings::android::app::NativeActivity::from(app.clone());
        let activity = activity.as_activity();
        let native_view = activity.find_view_by_id(env, android_bindings::ANDROID_R_ID_CONTENT);

        let decor_view = activity.get_window(env).get_decor_view(env);
        let content = decor_view.find_view_by_id(env, android_bindings::ANDROID_R_ID_CONTENT);
        let content = android_bindings::android::view::ViewGroup::from(*content);
        for i in 0..content.get_child_count(env) {
            let child = content.get_child_at(env, i);
            let class = env.find_class("android/view/SurfaceView").expect("Failed to get surface view");
            if env.is_instance_of(child, class)? {
                println!("FOUND THE surfaceview!");
            }
        }
        Ok(())
    }

    fn bad_idea_1(
        app: AndroidApp,
        env: JNIEnv,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ctx = ndk_context::android_context();
        let _vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

        let ctx = android_bindings::android::content::Context::from(unsafe {
            JObject::from_raw(ctx.context().cast())
        });
        let ctx = android_bindings::android::content::Context::default();
        // This works in java and android studio:
        // https://stackoverflow.com/a/39515370

        let activity = android_bindings::android::app::NativeActivity::from(app);
        let activity = activity.as_activity();
        let native_view = activity.find_view_by_id(env, android_bindings::ANDROID_R_ID_CONTENT);
        let window = activity.get_window(env);
        let ANDROID_R_COLOR_WHITE = 0x0106000a;
        window.set_background_drawable_resource(env, ANDROID_R_COLOR_WHITE);
        let decor_view = window.get_decor_view(env);
        let content = decor_view.find_view_by_id(env, android_bindings::ANDROID_R_ID_CONTENT);
        let content = android_bindings::android::view::ViewGroup::from(*content);
        content.remove_all_views(env);
        content.as_view().invalidate(env);
        content.as_view().request_layout(env);

        window.take_surface(env, android_bindings::android::view::SurfaceHolderCallback2::from(
                JObject::null()
        ));
        window.take_input_queue(env, android_bindings::android::view::InputQueueCallback::from(
                JObject::null(),
        ));

        let jchar_seq =
            android_bindings::java::lang::CharSequence::from(env.new_string("Text View from Rust!")?);

        let text_view = android_bindings::android::widget::TextView::new_1android_widget_text_view_landroid_content_context_2(
            env, ctx,
        );
        text_view.set_text_size_f(env, 24.);
        text_view.set_gravity(env, 17);
        text_view.set_text_color_i(env, 0xFF00000 as i32);

        text_view.set_text_keep_state_ljava_lang_char_sequence_2(env, jchar_seq);
        content.add_view_landroid_view_view_2(env, text_view.as_view());
        println!("Added text view to content view");
        //activity.set_content_view_landroid_view_view_2(env, text_view.as_view());

        Ok(())
    }


    /// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
    fn create_views_on_ui_thread(
        app: AndroidApp,
        env: JNIEnv,
        //native_window: &ndk::native_window::NativeWindow,
    ) -> Result<(), Box<dyn std::error::Error>> {
        clear_sufrace(app.clone());
        let config = app.config();
        log::debug!("CONFIG : {config:#?}");

        // Get a VM for executing JNI calls
        let ctx = ndk_context::android_context();
        let _vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

        let ctx = android_bindings::android::content::Context::from(unsafe {
            JObject::from_raw(ctx.context().cast())
        });
        // This works in java and android studio:
        // https://stackoverflow.com/a/39515370

        let activity = android_bindings::android::app::NativeActivity::from(unsafe {
            JObject::from_raw(app.activity_as_ptr().cast())
        });
        let activity = activity.as_activity();

        // TODO: use this call
        // activity.run_on_ui_thread(...)

        let jchar_seq =
            android_bindings::java::lang::CharSequence::from(env.new_string("Text View from Rust!")?);

        let text_view = android_bindings::android::widget::TextView::new_1android_widget_text_view_landroid_content_context_2(
            env, ctx,
        );

        text_view.set_text_keep_state_ljava_lang_char_sequence_2(env, jchar_seq);
        text_view.as_view().set_visibility(env, 0);

        // Set white background for visibility
        text_view
            .as_view()
            .set_background_color(env, 0xFFFF0000u32 as i32);

        // Set black text color (visible on white background)
        text_view.set_text_color_i(env, 0xFFFFFFFF_u32 as i32);
        text_view.set_text_size_f(env, 48.);
        text_view.as_view().set_elevation(env, 100.);
        text_view.as_view().set_alpha(env, 1.0);
        activity.set_content_view_landroid_view_view_2(env, text_view.as_view());

        /*
        let window = activity.get_window(env);
        window.set_format(env, -3);
        window.set_background_drawable(
            env,
            android_bindings::android::graphics::drawable::Drawable::from(JObject::null()),
        );

        let decor_view = window.get_decor_view(env);
        let decor_view = android_bindings::android::view::ViewGroup::from(*decor_view);

        //decor_view.add_view_landroid_view_view_2(env, text_view.as_view());
        text_view.as_view().bring_to_front(env);
        decor_view.as_view().invalidate(env);

        let content_frame = activity.find_view_by_id(env, 16908290); //android.R.id.content
        let content_frame = android_bindings::android::view::ViewGroup::from(*content_frame);
        let child_count = content_frame.get_child_count(env);

        if child_count > 0 {
            content_frame.remove_view_at(env, 0);
        }
        content_frame.add_view_landroid_view_view_2ii(env, text_view.as_view(), -2, -2);
        content_frame.as_view().set_background_color(env, 0x00000000);
        content_frame.as_view().invalidate(env);
        content_frame.as_view().request_layout(env);

        text_view.as_view().request_layout(env);
        text_view.as_view().invalidate(env);
        decor_view.as_view().request_layout(env);
        decor_view.as_view().invalidate(env);

        let width_spec = 1080 | (1 << 30);
        let height_spec = 2400 | (1 << 30);
        decor_view.as_view().measure(env, width_spec, height_spec);
        decor_view.layout(env, 0, 0, 1080, 2400);
        */

        println!("Added the text editor view");

        Ok(())
    }
}
impl ApplicationHandler<()> for App<'_> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        log::debug!("NEW EVENT: {cause:?}");
        if cause == winit::event::StartCause::Init {
        }
    }
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.create_views();
        //self.fix_surface_view();
        //log::debug!("RESUMED: {:?}", event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::debug!("WINDOW EVENT: {:?}", event);
        match event {
            winit::event::WindowEvent::RedrawRequested => {
             //   self.create_views()
            }
            winit::event::WindowEvent::Focused(true) => {
              //  self.create_views()
            }
            _other => {}
        }
    }
}

#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    };

    let mut event_loop: EventLoopBuilder<()> = EventLoop::with_user_event();
    event_loop.with_android_app(android_app.clone());
    let event_loop = event_loop.build().expect("Failed to build event loop");
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.expect("Failed to get vm");
    let env = vm
        .attach_current_thread_permanently()
        .expect("Failed to get env from vm");

    let mut winit_app = App { android_app, env };
    let _ = event_loop.run_app(&mut winit_app).expect("Fail to run app");
    log::debug!(
        "Android_main: {}",
        std::backtrace::Backtrace::force_capture()
    );
}

fn clear_sufrace(android_app: AndroidApp) {
    if let Some(native_window) = android_app.native_window() {
        native_window.set_buffers_geometry(
            0, 0,
            Some(HardwareBufferFormat::R8G8B8A8_UNORM),
        ).expect("Failed to set buffers geometry");
        let mut guard = native_window.lock(None).expect("Failed to get lock on window");
        // Get buffer info
        let width = guard.width() as usize;
        let height = guard.height() as usize;
        let stride = guard.stride() as usize;

        log::debug!("CLEAR SURFACE: Buffer: {}x{}, stride: {}", width, height, stride);
        if let Some(bytes) = guard.bytes() {
            bytes.fill(std::mem::MaybeUninit::new(0u8));
        }
    } else {
        log::debug!("FAILED TO GET THE NATIVE WINDOW");
    }
}
