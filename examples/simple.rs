use android_activity::{
    input::{InputEvent, KeyAction, KeyEvent, KeyMapChar, MotionAction},
    AndroidApp, InputStatus, MainEvent, PollEvent,
};
use android_bindings::{AndroidContentContext, AndroidWidgetEditText, AndroidWidgetTextView};
use jaffi_support::jni::{
    objects::{JObject, JValue},
    JavaVM,
};
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    info!("before hello world");
    println!("before hello world");
    let winow = app.native_window();
    let mut quit = false;
    let mut redraw_pending = true;
    let mut native_window: Option<ndk::native_window::NativeWindow> = None;
    while !quit {
        app.poll_events(
            Some(std::time::Duration::from_secs(1)), /* timeout */
            |event| {
                match event {
                    PollEvent::Wake => {
                        info!("Early wake up");
                    }
                    PollEvent::Timeout => {
                        info!("Timed out");
                        // Real app would probably rely on vblank sync via graphics API...
                        redraw_pending = true;
                    }
                    PollEvent::Main(main_event) => {
                        info!("Main event: {:?}", main_event);
                        match main_event {
                            MainEvent::SaveState { saver, .. } => {
                                saver.store("foo://bar".as_bytes());
                            }
                            MainEvent::Pause => {}
                            MainEvent::Resume { loader, .. } => {
                                if let Some(state) = loader.load() {
                                    if let Ok(uri) = String::from_utf8(state) {
                                        info!("Resumed with saved state = {uri:#?}");
                                    }
                                }
                            }
                            MainEvent::InitWindow { .. } => {
                                native_window = app.native_window();
                                redraw_pending = true;
                            }
                            MainEvent::TerminateWindow { .. } => {
                                native_window = None;
                            }
                            MainEvent::WindowResized { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::RedrawNeeded { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::InputAvailable { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::ConfigChanged { .. } => {
                                info!("Config Changed: {:#?}", app.config());
                            }
                            MainEvent::LowMemory => {}

                            MainEvent::Destroy => quit = true,
                            _ => { /* ... */ }
                        }
                    }
                    _ => {}
                }

                if redraw_pending {
                    if let Some(native_window) = &native_window {
                        redraw_pending = false;

                        // Handle input, via a lending iterator
                        match app.input_events_iter() {
                            Ok(mut iter) => loop {
                                info!("Checking for next input event...");
                                if !iter.next(|event| {
                                    match event {
                                        InputEvent::KeyEvent(key_event) => {
                                            /*
                                            let combined_key_char = character_map_and_combine_key(
                                                &app,
                                                key_event,
                                                &mut combining_accent,
                                            );
                                            info!("KeyEvent: combined key: {combined_key_char:?}")
                                            */
                                        }
                                        InputEvent::MotionEvent(motion_event) => {
                                            println!("action = {:?}", motion_event.action());
                                            match motion_event.action() {
                                                MotionAction::Up => {
                                                    let pointer = motion_event.pointer_index();
                                                    let pointer =
                                                        motion_event.pointer_at_index(pointer);
                                                    let x = pointer.x();
                                                    let y = pointer.y();

                                                    println!("POINTER UP {x}, {y}");
                                                    if x < 200.0 && y < 200.0 {
                                                        println!("Requesting to show keyboard");
                                                        app.show_soft_input(true);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        InputEvent::TextEvent(state) => {
                                            info!("Input Method State: {state:?}");
                                        }
                                        _ => {}
                                    }

                                    info!("Input Event: {event:?}");
                                    InputStatus::Unhandled
                                }) {
                                    info!("No more input available");
                                    break;
                                }
                            },
                            Err(err) => {
                                log::error!("Failed to get input events iterator: {err:?}");
                            }
                        }

                        info!("Render...");
                        dummy_render(native_window);
                    }
                }
            },
        );
    }

    info!("after hello world");
    println!("after hello world");
}
fn dummy_render(native_window: &ndk::native_window::NativeWindow) {
    unsafe {
        let mut buf: ndk_sys::ANativeWindow_Buffer = std::mem::zeroed();
        let mut rect: ndk_sys::ARect = std::mem::zeroed();
        ndk_sys::ANativeWindow_lock(
            native_window.ptr().as_ptr() as _,
            &mut buf as _,
            &mut rect as _,
        );
        // Note: we don't try and touch the buffer since that
        // also requires us to handle various buffer formats
        ndk_sys::ANativeWindow_unlockAndPost(native_window.ptr().as_ptr() as _);
    }
}

/// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
fn ndk_context_jni_test() -> Result<(), Box<dyn std::error::Error>> {
    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let env = vm.attach_current_thread()?;
    let context = AndroidContentContext::from(context);

    let text_editor =
        AndroidWidgetEditText::new_1android_widget_edit_text_landroid_content_context_2(
            *env, context,
        );

    // Since we aren't making JNI calls within the implementation of a native call from the JavaVM
    // we wrap the reference in an `AutoLocal` to make sure it will be deleted.
    let _int_ref = env.auto_local(
        env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(42)])
            .unwrap(),
    );

    Ok(())
}
