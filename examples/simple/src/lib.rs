use jni::{Env, EnvUnowned};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder},
    platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid},
};

pub struct App<'a> {
    android_app: AndroidApp,
    env: EnvUnowned<'a>,
}

impl<'a> App<'a> {
    fn create_views(&mut self) {
        println!("CREATING VIEWS");
        //let mut env_unowned = unsafe {EnvUnowned::from_raw(self.env_ptr) };
        let android_app = self.android_app.clone();

        // Initialize bindings within env scope
        let init_outcome = self.env.with_env(move |env| {
            let loader = jni::refs::LoaderContext::default();
            android_bindings::bindings::jni_init(env, &loader)
        }).into_outcome();;

        println!("ran JNI_INIT");
        // Get JavaVM for creating runnable
        let vm = self.env.with_env(|env| env.get_java_vm()).into_outcome();
        let vm = if let jni::Outcome::Ok(vm) = vm { vm } else { return; };

        println!("GOT THE VM");

        // Create a simple Runnable placeholder
        //let _runnable = android_bindings::bindings::java::lang::Runnable::default();



        // Create NativeActivity from the android_app pointer
        let activity_ptr = android_app.activity_as_ptr();
        let activity = self.env.with_env::<_, android_bindings::bindings::android::app::Activity, jni::errors::Error>(move |env| {
            let activity = unsafe {android_bindings::bindings::android::app::Activity::from_raw(
                env,
                activity_ptr.cast(),
            )};
            // NativeActivity extends Activity, use as_activity() to cast
            // let activity: android_bindings::bindings::android::app::Activity = _activity.as_activity();
            Ok(activity)
        }).into_outcome();
        let activity = if let jni::Outcome::Ok(activity) = activity { activity } else { return ;};
        println!("CREATED THE ACTIVITY");

        let runnable = self.env.with_env(|env| android_bindings::create_runnable(env, move || { println!("RUN ON UI THREAD");})).into_outcome();
        let runnable = if let jni::Outcome::Ok(runnable) = runnable { runnable } else { return };
        println!("CREATED THE RUNNABLE");
        let out = self.env.with_env(move |env| activity.run_on_ui_thread(env, runnable)).into_outcome();
        println!("RUN ON MAIN THREAD: {out:#?}");
        if let jni::Outcome::Ok(_) = out { } else {panic!("Did not run on the ui thread") };




        // Run on UI thread
        //let _ = _runnable;
    }

    #[allow(dead_code)]
    fn create_runnable<F: FnOnce() + Send + 'static>(&self, _f: F) -> android_bindings::bindings::java::lang::Runnable<'_> {
        // Create a simple Runnable implementation using the generated bindings
        android_bindings::bindings::java::lang::Runnable::default()
    }

    #[allow(dead_code)]
    fn drop_graphics(android_app: AndroidApp) {
        if let Some(native_window) = android_app.native_window() {
            unsafe {
                ndk_sys::ANativeWindow_release(native_window.ptr().as_ptr());
            }
            log::debug!("release_window: CALLED ANativeWindow_release");
        } else {
            log::error!("release_window: THERE IS NO NATIVE WINDOW");
        }
    }

    pub fn fix_surface_view(_android_app: AndroidApp, _env: Env) {
        // Placeholder - actual implementation needs methods not in generated bindings
    }

    #[allow(dead_code)]
    fn print_tree(_app: AndroidApp, _env: Env) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - actual implementation needs methods not in generated bindings
        Ok(())
    }

    #[allow(dead_code)]
    fn text_view(_env: Env) -> android_bindings::bindings::android::widget::TextView {
        // Placeholder - actual implementation needs methods not in generated bindings
        android_bindings::bindings::android::widget::TextView::default()
    }

    #[allow(dead_code)]
    fn create_views_on_ui_thread(
        _app: AndroidApp,
        _env: Env,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - actual implementation needs methods not in generated bindings
        Ok(())
    }
}

impl ApplicationHandler<()> for App<'_> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        log::debug!("NEW EVENT: {cause:?}");
        if cause == winit::event::StartCause::Init {}
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.create_views();
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
    log::debug!("Android main!");

    let mut event_loop: EventLoopBuilder<()> = EventLoop::with_user_event();
    event_loop.with_android_app(android_app.clone());
    let event_loop = event_loop.build().expect("Failed to build event loop");

    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) };
    let env_ptr = vm
        .attach_current_thread(|env: &mut Env| {
            Ok::<_, jni::errors::Error>((*env).get_raw())
        })
        .expect("Failed to get env from vm");
    let env_unowned = unsafe {EnvUnowned::from_raw(env_ptr) };

    let mut winit_app = App { android_app, env: env_unowned };
    let _ = event_loop.run_app(&mut winit_app).expect("Fail to run app");
    log::debug!(
        "Android_main: {}",
        std::backtrace::Backtrace::force_capture()
    );
}
